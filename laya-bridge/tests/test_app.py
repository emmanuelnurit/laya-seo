"""Wiring smoke test for laya_bridge.app: no real checkpoint is loaded here
(that needs network + a GPU/CPU inference pass, exercised instead by the
end-to-end smoke test against a real laya-bridge process, see README). This
only proves the FastAPI app, the Jev envelope, and the structured logging
middleware are wired together correctly, using a fake Router.
"""
import json
import logging

from fastapi.testclient import TestClient

from laya_bridge.app import create_app


class FakeRouter:
    """Stands in for laya.router.Router: same predict() signature and shape
    of result, no model weights involved."""

    loaded = ["multilingual"]
    loaded_revisions = {}

    def predict(self, state, questions, model=None, **kwargs):
        answers = {}
        for qid, q in questions.items():
            qtype = q.get("type")
            if qtype == "choice":
                first_key = next(iter(q["criteria"]))
                answers[qid] = {"choice": first_key, "confidence": 0.91, "probabilities": {first_key: 0.91}}
            elif qtype == "score":
                answers[qid] = {"score": 3.0, "confidence": 0.85}
            elif qtype == "noul":
                answers[qid] = {"noul": 0.12, "confidence": 0.88}
        return {
            "model": model or "multilingual",
            "answers": answers,
            "usage": {"input_tokens": 42, "output_tokens": len(questions)},
            "routing": {"model": model or "multilingual"},
        }


def test_health_reports_loaded_checkpoint():
    app = create_app(router=FakeRouter())
    client = TestClient(app)
    resp = client.get("/health")
    assert resp.status_code == 200
    assert resp.json()["status"] == "ok"


def test_systemone_returns_jev_shaped_answers():
    app = create_app(router=FakeRouter())
    client = TestClient(app)
    resp = client.post(
        "/v1/systemone",
        json={
            "model": "multilingual",
            "state": {"page": {"text": "Bonjour, comment configurer ce plugin ?"}},
            "questions": {
                "intent": {
                    "type": "choice",
                    "instructions": "Select the primary search intent.",
                    "criteria": {"informational": "how-to", "transactional": "buy now"},
                },
                "direct_answer": {"type": "noul", "instructions": "Direct answer up front?"},
            },
        },
    )
    assert resp.status_code == 200
    body = resp.json()
    assert body["answers"]["intent"]["choice"] == "informational"
    assert "confidence" in body["answers"]["intent"]
    assert 0.0 <= body["answers"]["direct_answer"]["noul"] <= 1.0


def test_systemone_call_is_logged_structured(caplog):
    app = create_app(router=FakeRouter())
    client = TestClient(app)
    with caplog.at_level(logging.INFO, logger="laya_bridge"):
        client.post(
            "/v1/systemone",
            json={
                "model": "multilingual",
                "state": {"page": {"text": "hello"}},
                "questions": {"urgent": {"type": "noul", "instructions": "Is this urgent?"}},
            },
        )
    lines = [r.message for r in caplog.records if r.name == "laya_bridge"]
    assert lines, "expected one structured log line for the /v1/systemone call"
    record = json.loads(lines[-1])
    assert record["path"] == "/v1/systemone"
    assert record["status"] == 200
    assert "latency_ms" in record
    assert record["decisions"]["urgent"]["kind"] == "noul"

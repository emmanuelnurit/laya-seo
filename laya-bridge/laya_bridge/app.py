"""FastAPI app: ``laya.serve`` wrapped with structured request logging.

``laya.serve`` already speaks the Jev ``/v1/systemone`` wire protocol
(see its module docstring) and forces the checkpoint jev-seo asks for via
``"model"`` in the request body -- jev-seo's laya-local backend always sends
``"multilingual"`` (see the fork's ``src/engine.rs::LAYA_LOCAL_MODEL``), so
what this module adds is:

1. A hard default of ``LAYA_MODELS=multilingual`` so the server never even
   loads the English checkpoint unless an operator explicitly overrides it --
   defense in depth against script blindness, on top of the client always
   pinning the model.
2. One structured JSON log line per request (path, status, latency, resolved
   model, per-question decision) so a scoring drift on this bridge can be
   diagnosed from logs alone, per the observability requirement on every
   Laya bridge call.
"""
import json
import logging
import os
import time
from typing import Any, Dict, Optional

_log = logging.getLogger("laya_bridge")


def _summarize_answer(answer: Any) -> Optional[Dict[str, Any]]:
    """Pull the one number worth logging out of a Jev-shaped answer object,
    without assuming which question type produced it."""
    if not isinstance(answer, dict):
        return None
    for key in ("noul", "score", "probability", "confidence"):
        if key in answer:
            return {"kind": key, "value": answer[key]}
    return None


def create_app(router: Optional[Any] = None):
    """Build the laya-bridge FastAPI app: laya.serve's app plus structured
    logging. Sets LAYA_MODELS=multilingual before laya.serve reads the
    environment, unless the caller already set it. ``router`` is forwarded to
    ``laya.serve.create_app`` so tests can inject a fake Router instead of
    loading a real checkpoint."""
    os.environ.setdefault("LAYA_MODELS", "multilingual")

    from fastapi import Response
    from starlette.middleware.base import BaseHTTPMiddleware

    from laya.serve import create_app as _laya_create_app

    app = _laya_create_app(router)

    class StructuredLoggingMiddleware(BaseHTTPMiddleware):
        async def dispatch(self, request, call_next):
            started = time.monotonic()
            response = await call_next(request)
            latency_ms = round((time.monotonic() - started) * 1000, 2)

            record: Dict[str, Any] = {
                "ts": time.time(),
                "path": request.url.path,
                "method": request.method,
                "status": response.status_code,
                "latency_ms": latency_ms,
            }

            if request.url.path == "/v1/systemone" and response.status_code == 200:
                chunks = [chunk async for chunk in response.body_iterator]
                body = b"".join(chunks)
                headers = {k: v for k, v in response.headers.items() if k.lower() != "content-length"}
                response = Response(
                    content=body,
                    status_code=response.status_code,
                    headers=headers,
                    media_type=response.media_type,
                )
                try:
                    payload = json.loads(body)
                    record["model"] = payload.get("model")
                    record["decisions"] = {
                        qid: _summarize_answer(ans)
                        for qid, ans in (payload.get("answers") or {}).items()
                    }
                except Exception:
                    _log.warning("could not parse /v1/systemone response for structured logging")

            _log.info(json.dumps(record, ensure_ascii=False))
            return response

    app.add_middleware(StructuredLoggingMiddleware)
    return app


def main() -> None:
    """Entry point for the ``laya-bridge`` console script."""
    import uvicorn

    logging.basicConfig(level=os.environ.get("LAYA_BRIDGE_LOG_LEVEL", "INFO"))

    host = os.environ.get("LAYA_HOST", "0.0.0.0")
    port = int(os.environ.get("LAYA_PORT", "8000"))
    uvicorn.run(create_app(), host=host, port=port, log_level=os.environ.get("LAYA_LOG_LEVEL", "info"))


if __name__ == "__main__":
    main()

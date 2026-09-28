Development


```bash
vendor/bin/phpunit                                   # module test suite
php Thelia cache:clear                               # after adding a hook, tool or command
```

Most tests run without a database: tools are tested against fake gateways, the runtime against a fake LLM client, the MCP server against an in-memory registry. The trigger detection queries (`Tests/EventListener`, `Tests/Service/Run`) are the exception — they exercise real Thelia events and Propel fixtures through `Thelia\Test\IntegrationTestCase`, so they need the Thelia test database (`php bin/test-prepare` from the project root) and run from there, e.g. `vendor/bin/phpunit -c phpunit.xml.dist local/modules/CommerceAgents/Tests/Service/Run/AgentRunQueueTest.php`.

Adding a tool:

1. Implement `Agent\Tool\ToolInterface` under `Tool/Shopping` or `Tool/Admin`, behind a gateway interface.
2. Alias the gateway interface to its Thelia implementation in `CommerceAgents::configureServices()`.
3. Write the unit test with a fake gateway. The tool is picked up automatically by the registry, the chats and the MCP server.

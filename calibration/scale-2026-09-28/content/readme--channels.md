Channels


A configurable agent can push a message out through `send_to_channel`, the single channel tool exposed to the model (capability `channels.send`), or, for customer-facing replies (e.g. review responses), `send_email_to_customer`. The channel — mail, Mattermost, Slack, or a third-party connector — is picked by the agent's `agent_channel` configuration, never by the model.

| `agent_channel.mode` | Behaviour |
|---|---|
| `draft` (default) | The message becomes a pending `agent_staged_change` (`target_type = channel_message`). Nothing leaves until an administrator approves it in **Proposed changes**; approval runs the same connector as `direct` would have. |
| `direct` | The connector sends the message immediately. |

Built-in connectors, each its own class under `Channel/Connector/`: `MailChannelConnector` (Thelia's mailer), `MattermostChannelConnector` and `SlackChannelConnector` (dedicated incoming-webhook integrations, sharing a common `AbstractWebhookChannelConnector` base rather than one generic webhook POST).

Connector settings (mail from-address, Mattermost/Slack webhook URLs, tokens…) are configured **once for the whole store**, not per agent, in the module configuration's **Channels** tab (`Controller/Admin/ChannelsConfigController.php`, routes `commerceagents_channels_test` and `commerceagents_channels_save`): pick a connector, fill its settings form, test the connection, save. The agent wizard's Channels step only checks which connectors an agent is allowed to use; it links to this panel for the actual settings. Everything is encrypted with sodium `secretbox` before it reaches the database (`ChannelSettingsEncryptor`, key derived from `kernel.secret`); the gateways decrypt it transparently when a tool or applier needs it. The mail connector requires an explicit sender address (`store_email`) — sending without one raises a `ChannelException` rather than falling back to a transport default, both when sending for real and when testing the connection from the Channels tab.

Registering a connector from any module: implement `Channel\ChannelConnectorInterface` under an autowired/autoconfigured service — the `commerce_agents.channel_connector` tag is applied automatically to every implementation, exactly like `ToolInterface` and `ChangeApplierInterface`, so `ChannelConnectorRegistry` picks it up with no further wiring.

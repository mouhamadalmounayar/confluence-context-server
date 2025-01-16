# Confluence Context Server
This Zed extension provides a context server for Confluence.
It allows users to add the content of confluence pages as contexts in the assistant.
You can find the repository of the confluence model context server [here](https://github.com/mouhamadalmounayar/mcp-confluence)
# Configuration
In order to use this extension, you need to specify the following settings in the `settings.json` file:
```json
"context_servers": {
    "confluence-context-server": {
      "settings": {
        "api_token": ,
        "domain_name": ,
        "email": ,
      }
    }
  }
```
[how to generate an api token](https://support.atlassian.com/atlassian-account/docs/manage-api-tokens-for-your-atlassian-account/)

# Slash Commands
- `/confluence-page <page-id>`: This command will add the context of the confluence page with the given `page-id` to the assistant.

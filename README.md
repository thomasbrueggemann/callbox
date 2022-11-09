# ☎️ Callbox

Web hook API that triggers Twilio phone calls

A call will be triggered with a POST request to the base URL of the service

## Configuration

The service is configured via environment variables.
The following variables exist and are all mandatory to set:

| ENV               | Description                                                        |
| ----------------- | ------------------------------------------------------------------ |
| TWILIO_ACCOUNT_ID | Account ID for your Twilio account to trigger calls from           |
| TWILIO_AUTH_TOKEN | Matching Twilio authentication token                               |
| BASE_URL          | The base url where this service is deployed to. No trailing slash! |
| CALL_FROM         | Phone number for outgoing call                                     |
| CALL_TO           | Outgoing call recipient number                                     |

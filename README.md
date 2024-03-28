# channels-cli

## Installation

Visit [https://channels.devpulsehq.com] for more information on how to create local webhook channels.

## Usage

```bash
brew tap DevPulseHQ/tap
```

```bash
brew install channels-cli
```

To create a new configuration file, run the following command:

```bash
channels-cli init
```

Visit: [https://channels.devpulsehq.com] to create a new webhook channel or a set of webhook channels.

Once you have created a webhook channel, you can add the webhook channel to the configuration file.

```yaml
api_key: test_api_key
channels:
  - name: channel
    channel_id: 2dyxqmRqgYfP4uZiiUadVrbqccN
    target: http://example.com/channel
  - name: channel2
    channel_id: 2dyxqmRqgYfP4uZiiUadVrbqccN
    target: http://example.com/channel/2
```

Then run the following command to send a message to the webhook channel:

```bash
channels-cli
```

or if the configuration file is different to channels_config.yml

```bash
channels-cli --config={config-file-name}.yml
```

To see all the available options, run the following command:

```bash
channels-cli --help
```

### Development

If you want to adjust the logging levels:

```bash
-LOG_LEVEL=debug cargo run -- --config=tests/config.yml
```

### Contributing

If you want to contribute to the project, please follow the steps below:

- Fork the repository
- Create a new branch
- Make your changes
- Push the changes to your fork
- Create a pull request to the main repository

I will review. If everything is fine, I will merge the changes into the main repository.

## License

Just use it or copy it - who cares. It's just a CLI tool.

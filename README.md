# InfoMaker

A simple builder for Telegram Bots

### About

This project allows you to simply create your own info-based Telegram Bot

#### What means 'info-based'?

We call Telegram Bots containing static text data and showing it via simple inline menu `info-based`

### How can I create my own info-based bot?

1. Create Telegram Bot thought @BotFather
2. Write your config file
3. Start the application providing config path as `--states` and Bot API Token as `--token`

### What the config is? 
It's pretty simple. The config is a JSON file with the array of `states`. `State` is an object which has `name`, `text` and `btns`. `name` is the state name used to switch the state, `text` is a text displayed in the Telegram Message at the moment, and `btns` is the array of `buttons`. `Button` is an object which contains `text` to be displayed and `on_click` event, which is an object and may be `"set_text": "{text}"` or `"switch_state": "{state name}"`. Here is the basic example of a config:

```json
[
    {
        "name": "default",
        "text": "Hi! It's an example",
        "btns": [
            {
                "text": "Hi! Great!",
                "on_click": {
                    "set_text": "Good to hear you like this"
                }
            },
            {
                "text": "Hmm, tell me more..",
                "on_click": {
                    "switch_state": "more"
                }
            }
        ]
    },
    {
        "name": "more",
        "text": "You can read README!",
        "btns": [
            {
                "text": "Ok!",
                "on_click": {
                    "switch_state": "default"
                }
            }
        ]
    }
]
```
# Description
GPT API applied to shell via bash script

# Setup
Run the **setup script** in order to configurate script:

```bash
$ ./setup.sh
```

Reload your shell configuration file:

EX (zsh):
```bash
$ source ~/.zshrc
```

# Usage
To run the script, use the **ask** command followed by your promp

### Command
```bash 
$ ask "who are you?"
```

### Response
```bash
"I am a helpful Linux shell assistant here to assist you with any questions or problems you may have. I can provide information, guidance, and execute commands in the Linux shell environment. How can I assist you today?"
```

## Flags
There are optional flags for overwriting some configuration fields

- **-t**: Changes max tokens returned by request (_MAX\_TOKENS_)
- **-m**: Changes model used in request (_MODEL_)
- **-T**: Changes temperature for promp processing (_TEMPERATURE_)
- **-r**: Returns raw code, without any extra information or formatting

the **-h** flag can be used to read about available flags directly from the script.

### Example
```bash
$ ask -t 300 -m gpt-4 -T 0.7 "tell me about linux"
```

## Processing standard input
The script will append stdin to the prompt, yielding results as following:

### Command
```bash
$ cat gptask.sh | ask "what is this?"
```
or
```bash
$ ask "what is this?" < gptask.sh
```

### Response
```bash
"This is a bash script that interacts with the OpenAI API to generate responses based on user input. The script reads from standard input or command line arguments, sends a request to the OpenAI API, and prints the response. It also handles configuration variables, provides help information, and logs previous interactions."
```

## Generating code
Using the `-r` flag, code can easily be outputed to a file.

```bash
$ ask -r "javascript code for consuming an SSE route, including imports" > sse_connection.js
```

## Repeating answers
You can repeat the last answer to stdout with the command
```bash
ask rpt
```

# Configuration
## Configuration file
Configuration options are stored in the `config` file, created during the setup process. If any configuration is missing from the `config` file, the one contained in `config.example` will be used instead.

Configuration parameters are:
- **KEY**: OpenAI API key
- **MODEL**: Model used in the request
- **MAX_TOKENS**: Limit of generated response tokens
- **TEMPERATURE**: Response temperature
- **MAX_CHAT_MEMORY**: Number of exchanged kept during the conversation for context
- **KNOW_CURRENT_DIR**: Defines if current dir will be informed durring conversation

Configuration variables from `config` and/or `config.example` files are always overwriten by flags sent when calling the script.

## Editing configurations
Besides editing the `config` file directly, you can also use the CLI to do it.

Run the command
```bash
ask config
```
to automatically open your default terminal editor and make changes to your configuration file.


You can also just check the current configuration with the command:
```bash
ask config ls
```
which returns every configurations besides your secret key:
```
MODEL=gpt-4o
MAX_TOKENS=200
TEMPERATURE=0.2
MAX_CHAT_MEMORY=5
KNOW_CURRENT_DIR=true
```
Or use the flag `-a`, which also returns the key:
```
KEY=[YOUR SECRET KEY]
MODEL=gpt-4o
MAX_TOKENS=200
TEMPERATURE=0.2
MAX_CHAT_MEMORY=5
KNOW_CURRENT_DIR=true
```

# Chat history management
**ask** allows multiple chats to be created and managed, besides the default `chat1`.

## Changing current chat
Through this command you can change your current used chat. If a chat whith that name does not exist, it will be created.
```bash
ask hist ch [CHAT_NAME]
```

## Listing chats
```bash
ask hist ls
```

## Checking chat content
```bash
ask hist show [CHAT_NAME=CURRENT_CHAT]
```

# Cleaning previous conversations
Previous promps and responses are kept in the 'hist/[YOUR CURRENT SESSION].json' file. You can clean all previous conversations by deleting the file or simply running the command
```bash
ask cln [CHAT_NAME=CURRENT_CHAT]`
```

# References
- [OpenAI API Docs](https://platform.openai.com/docs/api-reference/introduction)

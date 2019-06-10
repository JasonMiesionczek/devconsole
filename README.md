# devconsole

Tool to run multiple development processes in parallel. 

## Example

```
devconsole tasks.json
```
tasks.json:
```json
[
    {
        "name": "web",
        "working_dir": "/home/jason/Projects/MusicManager/clients/web",
        "binary": "yarn",
        "color": "bright_blue",
        "args": [
            "start"
        ],
        "env": {},
        "group": 1
    },
    {
        "name": "db",
        "working_dir": "/home/jason/Projects/MusicManager",
        "binary": "docker-compose",
        "color": "red",
        "args": [
            "up"
        ],
        "env": {},
        "group": 0
    },
    {
        "name": "xvfb",
        "working_dir": "/home/jason/Projects/MusicManager/server",
        "binary": "Xvfb",
        "color": "yellow",
        "args": [
            ":99",
            "-screen",
            "0",
            "1024x768x16"
        ],
        "env": {},
        "group": 0
    },
    {
        "name": "task-runner",
        "working_dir": "/home/jason/Projects/MusicManager/server",
        "binary": "cargo",
        "color": "green",
        "args": [
            "watch",
            "-x",
            "run --bin task-runner"
        ],
        "env": {},
        "group": 1
    },
    {
        "name": "api",
        "working_dir": "/home/jason/Projects/MusicManager/server",
        "binary": "cargo",
        "color": "magenta",
        "args": [
            "watch",
            "-x",
            "run --bin api"
        ],
        "env": {},
        "group": 1
    },
    {
        "name":"static",
        "working_dir": "/home/jason/Projects/MusicManager/static",
        "binary": "docker-compose",
        "color": "cyan",
        "args": ["up"],
        "env": {
            "MUSIC_DIR": "/home/jason/Music",
            "IMAGE_DIR": "/home/jason/Images"
         },
         "group": 0
    }
]
```

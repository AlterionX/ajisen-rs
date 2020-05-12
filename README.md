![Tests and Build](https://github.com/yoloVoe/ajisen-rs/workflows/Run%20Tests%20and%20Publish/badge.svg)

# Ajisen
Ajisen is a general-purpose Discord bot.

Add it to your server by clicking [this link](https://discordapp.com/api/oauth2/authorize?client_id=462110877991305217&permissions=27712&scope=bot).

# Commands

## `help`

### Usage

`~help` to see a list of commands.

`~help [command_name]` to see details about a specific command.
For example, `~help roll` will show the details of the `roll` command.

## `roll`

### Usage
`~roll NdM`, where `N` is number of dice and `M` is number of sides in each dice.

Example: `~roll 1d6`

### Available
In DM and guilds.

## `poll`

### Usage
Start a poll where users can vote for various options with reactions.
If no options are provided, the question is assumed to be a yes or no question.

Example: `~poll "How's the weather today?" "Good." "Ok" "Bad"`

### Available
In DM and guilds.

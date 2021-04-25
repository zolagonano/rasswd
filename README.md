# rasswd

rasswd is a simple password generator implemented in rust.

## Installation:

### From cargo:

```bash
cargo install rasswd
```

### From the source code:

1. Clone the repository:

```bash
git clone https://github.com/zolagonano/rasswd.git
```

2. Change your working directory to source code directory:

```bash
cd rasswd
```

3. Compile and run:

```bash
cargo build --release
cargo run
```

## Usage
By default, if you run rasswd you will get a 45 character password that includes all characters.

But if you want to customize it you can use these letters in the first argument: 

|letter|meaning|
|---|---|
|`l`|lowercase letters|
|`L`|uppercase letters|
|`p`|punctuations|
|`d`|digits|

You can mix these letters and generate a custom password, for example:

```bash
rasswd Ldp
```

That command above will generate a password that includes uppercase letters, digits, and punctuations.

To change the length of the password you can give the second argument to it, for example:

```bash
rasswd Ldp 20
```

## Contribute:
All contributions are welcome but if you don't know what you can do look at this list:

- Open an issue if you find a bug.
- Open an issue if you have a suggestion.
- Fix bugs and send pull requests.
- Share it with your friends.
- And anything else you think will help this project :).
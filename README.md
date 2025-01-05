# Nimo

Nimo is a simple and efficient CLI tool written in Rust, designed to fetch any directory or file from a GitHub repository with ease. Its name is derived from the Japanese word "nimotsu" (にもつ), meaning "luggage," reflecting its purpose of "carrying" files conveniently for project initialization. 📦✨

## Features
- **Fetch specific files or directories** from a GitHub repository. 🚀
- Supports specifying a branch (default is `main`). 🌿
- Allows setting a custom output directory for fetched content. 🗂️

## Usage
```bash
nimo <path> [OPTIONS]
```

### Arguments
- `path` (required): The relative path to the file or directory you want to download from the repository, in the format `user/repo/file_or_directory_path`, e.g., `username/my-repo/nvim/init.lua`. 📁

### Options
- `-o`, `--output` (optional): Specify the output directory where the file or directory will be saved. If not provided, the current working directory will be used. 📂
- `-b`, `--branch` (optional): Specify the branch to download from. If not provided, the default branch is `main`. 🌲

### Example Commands
1. **Fetch a single file:**
   ```bash
   nimo username/my-repo/nvim/init.lua
   ```
   This command fetches the `nvim/init.lua` file from the default branch (`main`) of the `my-repo` repository owned by `username` and saves it in the current directory. 📜

2. **Specify an output directory:**
   ```bash
   nimo username/my-repo/nvim/init.lua -o ./config
   ```
   This fetches the file and saves it in the `./config` directory. 📥

3. **Specify a branch:**
   ```bash
   nimo username/my-repo/nvim/init.lua -b dev
   ```
   This fetches the file from the `dev` branch of the repository instead of the default `main` branch. 🔀

## How It Works
Nimo takes advantage of GitHub’s raw content URLs to fetch files or directories. Given a valid `path`, it constructs the appropriate URL, downloads the content, and saves it to the specified output location. 🌐📩

## Contributing
Contributions are welcome! If you find any issues or have ideas for new features, feel free to submit an issue or a pull request. 🙌✨

## License
This project is licensed under the [MIT License](LICENSE). 📝

---

Happy hacking with Nimo! 😄🎉


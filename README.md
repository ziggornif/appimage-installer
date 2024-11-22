# appimg-installer

A CLI to install AppImage files on my linux distro.

I use this tool to put AppImage applications in a target directory and generate the associated desktop file to see them in the UI application menu.

The project is an excuse to do some Rust and could very well be written more simply in shell script (and surely lighter) 😁.

## Installation

```sh
git clone
cargo build --release
sudo cp target/release/appimg-installer /usr/local/bin
```

## Usage

Run this command with your AppImage file.

```sh
appimg-installer -f myApp.AppImage
```

Answer to questions :

```
Welcome to AppImage desktop installer
Enter the application name (ex: FreeCAD):
Demo
Enter the application description:
My awesome app
Enter the icon file path (ex: ./freecad.svg):
./demo.svg
Enter the application target directory (default: $USER/Apps):
/home/user/Apps
Enter the application category (ex: Graphics):
Graphics
Icon has been copied in /home/user/.local/share/icons/demo.svg directory
Application installed in /home/user/Apps/Demo.AppImage directory
```

You should see the application in the applications menu. 

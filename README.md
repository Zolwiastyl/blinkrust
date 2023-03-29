# blinkrust

Trying to wrap my head around embedded systems
Having fun on Pico W

## Setting up debugging by pico probe and gdb

### Two main commands

setting them up described below

run this in openocd repo:

```
openocd -f interface/cmsis-dap.cfg -f target/rp2040.cfg -c "adapter speed 5000"
```

do not close the tab

and run this as build command:

```
cargo build
```

and then this to run `gdb`:

```
gdb-multiarch -q -ex "target extended-remote :3333" target/thumbv6m-none-eabi/debug/blinkerust
```

### What had to do to run it

```
sudo apt-get install \
  gdb-multiarch \
  minicom
```

```
sudo nano /etc/udev/rules.d/99-microbit.rules
```

(probably this could be `picoprobe.rules` insted of ripping off from rust embedded)
and paste following

```
# Raspberry Pi Picoprobe
ATTRS{idVendor}=="2e8a", ATTRS{idProduct}=="0004", MODE:="0666"
```

then run:

```
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Config the `embed.toml` as in source code

get my own openocd and build it:

```
git clone https://github.com/raspberrypi/openocd.git --branch rp2040 --depth=1
cd openocd
./bootstrap
./configure --disable-werror
make -j4
```

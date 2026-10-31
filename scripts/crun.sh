#!/bin/sh

# ====================================================
# =                      CRUN                        =
# =  Utility script to launch a rust executable      =
# = without using cargo run that would take some     =
# =         overhead time for building               =
# =                                                  =
# =                 by Alice Sonolet                 =
# ====================================================

# Fix for Git Bash: Remove GNU coreutils from PATH to avoid
# conflict with MSVC linker. The GNU link.exe creates hard links,
# while MSVC link.exe is the C/C++ linker.
# This is only needed when using Git Bash on Windows with MSVC target
if [ -n "$MSYSTEM" ] || [ -f /usr/bin/link.exe ]; then
    # Save original PATH for tput (if needed)
    ORIGINAL_PATH=$PATH
    
    # Remove /usr/bin and /mingw64/bin from PATH temporarily
    export PATH=$(echo $PATH | tr ':' '\n' | grep -v "^/usr/bin$" | grep -v "^/mingw64/bin$" | tr '\n' ':')
    
    # Prepend Visual Studio paths if they exist
    VS_LINKER_PATH="C:/Program Files (x86)/Microsoft Visual Studio/2022/Community/VC/Bin"
    if [ -d "$VS_LINKER_PATH" ]; then
        export PATH="$VS_LINKER_PATH:$PATH"
    fi
    
    VS_LINKER_PATH="C:/Program Files (x86)/Microsoft Visual Studio/2019/Community/VC/Bin"
    if [ -d "$VS_LINKER_PATH" ]; then
        export PATH="$VS_LINKER_PATH:$PATH"
    fi
    
    VS_LINKER_PATH="C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Bin"
    if [ -d "$VS_LINKER_PATH" ]; then
        export PATH="$VS_LINKER_PATH:$PATH"
    fi
fi

# Try to use tput for colors, but don't fail if it's not available
if command -v tput >/dev/null 2>&1; then
    txtbld=$(tput bold)             # Bold
    bldred=${txtbld}$(tput setaf 1) #  red
    txtrst=$(tput sgr0)             # Reset
else
    txtbld=""
    bldred=""
    txtrst=""
fi

# Check that the project exists
if [ ! -f "$1/Cargo.toml" ]; then
	echo "$bldred ERROR : No cargo file found on $1 $txtrst"
	exit 1
fi

for var in ${@:2}
do
	if [ "$var" == "-b" ]; then
		currentpwd=$PWD
		cd $1
		cargo build --release
		exitCode=$?
		cd $currentpwd
		if [ "$exitCode" != "0" ]; then
			exit 1
		fi
	fi
done


# Finds the project name
while read currentline; do
 	substr=${currentline:0:6}
	if [ "$substr" = "name =" ]; then 
  		break
	fi;
done < $1/Cargo.toml
projectName=${currentline:8:-1}

# Finds the executable
exePath="$1/target/release/$projectName"
if [ ! -f "$exePath" ]; then
	echo "$bldred ERROR : Could not find executable at $exePath $txtrst"
	exit 1
fi

CMD_ARGS=""
for var in ${@:2}
do
	if [ "$var" == "-b" ]; then
		:  # Do nothing
	else
		CMD_ARGS="$CMD_ARGS $var"
	fi
done

$exePath $CMD_ARGS
exitCode=$?
if [ "$exitCode" != "0" ]; then
	exit $exitCode
fi

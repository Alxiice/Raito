#!/bin/sh

# ====================================================
# =                      CRUN                        =
# =  Utility script to launch a rust executable      =
# = without using cargo run that would take some     =
# =         overhead time for building               =
# =                                                  =
# =                 by Alice Sonolet                 =
# ====================================================

txtbld=$(tput bold)             # Bold
bldred=${txtbld}$(tput setaf 1) #  red
txtrst=$(tput sgr0)             # Reset

# Check that the project exists
if [ ! -f "$1/Cargo.toml" ]; then
	echo -e "$bldred ERROR : No cargo file found on $1 $txtrst"
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
	echo -e "$bldred ERROR : Could not find executable at $exePath $txtrst"
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
#!/bin/bash

function finish {
	killall vanity
}

if [ "$#" -ne 1 ]; then
	echo "Usage: $0 <threads> " >&2
	exit 1
fi

trap finish EXIT

echo " = Prefixes = "
cat prefixes.txt

i=0
while [ $i -lt $1 ]
do
	./vanity &
	i=`expr $i + 1`
done

echo 'Press <Ctrl+C> to kill...'
wait

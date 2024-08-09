#!/bin/bash

SCRIPT_PATH=$(dirname $0)
HIST_DIR=$SCRIPT_PATH/hist

# creates session file if not exists and loads contents
if [ ! -f $SCRIPT_PATH/.session ]; then
    echo 'CURRENT_CHAT=chat1' > $SCRIPT_PATH/.session
fi

export $(grep -v '^#' $SCRIPT_PATH/.session | xargs)

if [ ! -f $HIST_DIR/$CURRENT_CHAT.json ]; then
    echo '{"hist":[]}' > $HIST_DIR/$CURRENT_CHAT.json
fi

# imports config variables
export $(grep -v '^#' $SCRIPT_PATH/config.example | xargs)

if [ -f $SCRIPT_PATH/config ]; then
	export $(grep -v '^#' $SCRIPT_PATH/config | xargs)
fi

CURRENT_DIR=
if [ $KNOW_CURRENT_DIR = "true" ]; then
	CURRENT_DIR="You are currently in the $PWD directory."
fi

# retrieves stdin if exists
STDIN=
if [ ! -t 0 ]; then
	while IFS= read -r line; do
		STDIN="$STDIN\n$(echo -E $line)"
	done
fi

change_config() {
	if [ "$1" = "ls" ]; then
		offset=2
		if [ "$2" = "-a" ]; then
			offset=$((offset - 1))
		fi

		tail -n +$offset $SCRIPT_PATH/config
		return 0
	fi
	${EDITOR:-vi} $SCRIPT_PATH/config
}

case $1 in
    hist) source $SCRIPT_PATH/hist.sh;;
    rpt) $SCRIPT_PATH/gptask.sh hist rpt $2;;
    cln) $SCRIPT_PATH/gptask.sh hist rm $2;;
    config) change_config $2 $3;;
    *) source $SCRIPT_PATH/ask.sh;;
esac

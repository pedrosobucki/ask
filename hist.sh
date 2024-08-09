#!/bin/bash

help_info() {
	echo -e "Flags:\n\t-h: Display help info\n\t-l: Returns last conversation\n\t-r: Repeats last response"
	exit 0
}

parse() {
    PARSED=$(echo -E $1 | sed 's/\\\"/\"/g')
    PARSED=${PARSED:1:-1}
    echo -E $PARSED
}

get_json_history() {
    if [ ! -f $HIST_DIR/$1.json ]; then
        echo "Chat \"$1\" not found."
        exit 1
    fi

    cat $HIST_DIR/$1.json
}

list_history() {
    echo "Available chat history:"
    ls $HIST_DIR -1 | sed 's/\.json$//'  
}

show_history() {
    CHAT=$1

    if [ ! -f $HIST_DIR/$CHAT.json ]; then
        echo "Chat \"$CHAT\" not found."
        exit 1
    fi

    cat $HIST_DIR/$CHAT.json | jq
}

repeat_answer() {
    CHAT=$1

    if [ ! -f $HIST_DIR/$CHAT.json ]; then
        echo "Chat \"$CHAT\" not found."
        exit 1
    fi

    ANSWER=$(cat $HIST_DIR/$CHAT.json | jq '.hist | .[-1] | .content')
    ANSWER=$(parse "$ANSWER")
    echo -e $ANSWER
}

rm_history() {
    if [ ! -f $HIST_DIR/$1.json ]; then
        echo "Chat \"$1\" not found."
        exit 1
    fi

    rm $HIST_DIR/$1.json
}

change_chat() {
    if [ ! -f $HIST_DIR/$1.json ]; then
        echo '{"hist":[]}' > $HIST_DIR/$1.json
    fi

    echo "CURRENT_CHAT=$1" > $SCRIPT_PATH/.session
}

case $2 in
    ls) list_history;;
    ch) change_chat ${3:-$CURRENT_CHAT};;
    rm) rm_history ${3:-$CURRENT_CHAT};;
    show) show_history ${3:-$CURRENT_CHAT};;
    rpt) repeat_answer ${3:-$CURRENT_CHAT};; 
    *) list_history;;
esac
source "$SHDIR/exit.sh"

# Local configuration that is specific to each machine.
LOCAL_ZSH="$ZDOTDIR/.local.zsh" && test -f $LOCAL_ZSH && source $LOCAL_ZSH

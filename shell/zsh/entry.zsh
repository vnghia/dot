source "$SHDIR/entry.sh"

# History
export HISTFILE="$LOCALDIR/.zsh_history"
export HISTSIZE=999999999
export SAVEHIST=999999999
setopt HIST_EXPIRE_DUPS_FIRST
setopt HIST_IGNORE_DUPS
setopt HIST_IGNORE_ALL_DUPS
setopt HIST_IGNORE_SPACE
setopt HIST_FIND_NO_DUPS
setopt HIST_SAVE_NO_DUPS

# Compdump
export ZSH_COMPDUMP="$LOCALDIR/.zsh_compdump"

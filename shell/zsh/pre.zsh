source "$SHDIR/pre.sh"

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
setopt INC_APPEND_HISTORY

# Compdump
export ZSH_COMPDUMP="$LOCALDIR/.zsh_compdump"

# Rye
RYE_ENV="$RYE_HOME/env" && test -f $RYE_ENV && source $RYE_ENV

# Local configuration that is specific to each machine.
LOCAL_PRE_ZSH="$ZDOTDIR/.local.pre.zsh" && test -f $LOCAL_PRE_ZSH && source $LOCAL_PRE_ZSH

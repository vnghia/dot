# Ls colors
export CLICOLOR=1
export LSCOLORS="gxfxbxdxcxegedabagacad"
export LS_COLORS="di=36:ln=35:so=31:pi=33:ex=32:bd=34;46:cd=34;43:su=30;41:sg=30;46:tw=30;42:ow=30;43"
alias ls="ls --color=auto"

# Local configuration that is specific to each machine.
LOCAL_POST_SH="$SHDIR/.local.post.sh" && test -f $LOCAL_POST_SH && source $LOCAL_POST_SH

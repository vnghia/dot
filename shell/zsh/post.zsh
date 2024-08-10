source "$SHDIR/post.sh"

# Local configuration that is specific to each machine.
LOCAL_POST_ZSH="$ZDOTDIR/.local.post.zsh" && test -f $LOCAL_POST_ZSH && source $LOCAL_POST_ZSH

# Start zellij after everything to avoid missing configuration.
# Don't start zellij if one of either two envs below is not empty.
[[ -n $DISABLE_ZELLIJ || -n $VSCODE_INJECTION ]] || eval "$(zellij setup --generate-auto-start zsh)"

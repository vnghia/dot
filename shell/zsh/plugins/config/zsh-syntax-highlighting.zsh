typeset -A ZSH_HIGHLIGHT_STYLES

ZSH_HIGHLIGHT_STYLES["unknown-token"]="fg=160"
ZSH_HIGHLIGHT_STYLES["reserved-word"]="fg=130"
ZSH_HIGHLIGHT_STYLES["alias"]="fg=84"
ZSH_HIGHLIGHT_STYLES["builtin"]="fg=84"
ZSH_HIGHLIGHT_STYLES["command"]="fg=84"
ZSH_HIGHLIGHT_STYLES["function"]="fg=84"

ZSH_HIGHLIGHT_STYLES["commandseparator"]="fg=250"
ZSH_HIGHLIGHT_STYLES["redirection"]="fg=250"

ZSH_HIGHLIGHT_STYLES["path"]="fg=45"
ZSH_HIGHLIGHT_STYLES["globbing"]="fg=33"

ZSH_HIGHLIGHT_STYLES["command-substitution-delimiter"]="fg=78"
ZSH_HIGHLIGHT_STYLES["process-substitution-delimiter"]="fg=78"
ZSH_HIGHLIGHT_STYLES["back-quoted-argument-delimiter"]="fg=78"

QUOTED_ARGUMENT=178
ZSH_HIGHLIGHT_STYLES["single-quoted-argument"]="fg=$QUOTED_ARGUMENT"
ZSH_HIGHLIGHT_STYLES["double-quoted-argument"]="fg=$QUOTED_ARGUMENT"
ZSH_HIGHLIGHT_STYLES["dollar-quoted-argument"]="fg=$QUOTED_ARGUMENT"
ZSH_HIGHLIGHT_STYLES["dollar-double-quoted-argument"]="fg=$QUOTED_ARGUMENT"
QUOTED_ARGUMENT_UNCLOSED=248
ZSH_HIGHLIGHT_STYLES["single-quoted-argument-unclosed"]="fg=$QUOTED_ARGUMENT_UNCLOSED"
ZSH_HIGHLIGHT_STYLES["double-quoted-argument-unclosed"]="fg=$QUOTED_ARGUMENT_UNCLOSED"
ZSH_HIGHLIGHT_STYLES["dollar-quoted-argument-unclosed"]="fg=$QUOTED_ARGUMENT_UNCLOSED"
ZSH_HIGHLIGHT_STYLES["dollar-double-quoted-argument-unclosed"]="fg=$QUOTED_ARGUMENT_UNCLOSED"

ZSH_HIGHLIGHT_STYLES["single-hyphen-option"]="fg=180"
ZSH_HIGHLIGHT_STYLES["double-hyphen-option"]="fg=228"

ZSH_HIGHLIGHT_STYLES["default"]="fg=253"

source "$SHDIR/directory.sh"
source "$SHDIR/path.sh"
source "$SHDIR/config.sh"

# Local configuration that is specific to each machine.
LOCAL_PRE_SH="$SHDIR/.local.pre.sh" && test -f $LOCAL_PRE_SH && source $LOCAL_PRE_SH

pub fn shell_hook(shell: &str) {
    let hook = match shell {
        "zsh" | "bash" => r#"
awsx() {
    local output
    output=$(command awsx "$@" 2>&1)
    local exit_code=$?
    # Source any export lines, print the rest
    while IFS= read -r line; do
        case "$line" in
            export\ *) eval "$line" ;;
            *) echo "$line" ;;
        esac
    done <<< "$output"
    return $exit_code
}
"#,
        "fish" => r#"
function awsx
    set -l output (command awsx $argv 2>&1)
    set -l exit_code $status
    for line in $output
        if string match -q 'export *' -- $line
            set -l var (string replace 'export ' '' -- $line)
            set -l key (string split '=' -- $var)[1]
            set -l val (string split '=' -- $var)[2]
            set -gx $key $val
        else
            echo $line
        end
    end
    return $exit_code
end
"#,
        _ => {
            eprintln!("Unsupported shell: {shell}. Use zsh, bash, or fish.");
            std::process::exit(1);
        }
    };
    print!("{hook}");
}

pub fn prompt_hook(shell: &str) {
    let hook = match shell {
        "zsh" => r#"
_awsx_prompt() {
    [[ -n "$AWSX_CONTEXT" ]] && echo "☁️ $AWSX_CONTEXT"
}
RPROMPT='$(_awsx_prompt) '"$RPROMPT"
"#,
        "bash" => r#"
_awsx_prompt() {
    [[ -n "$AWSX_CONTEXT" ]] && printf "☁️ %s " "$AWSX_CONTEXT"
}
PS1='$(_awsx_prompt)'"$PS1"
"#,
        "fish" => r#"
function fish_right_prompt
    if set -q AWSX_CONTEXT
        echo "☁️ $AWSX_CONTEXT"
    end
end
"#,
        _ => "",
    };
    print!("{hook}");
}

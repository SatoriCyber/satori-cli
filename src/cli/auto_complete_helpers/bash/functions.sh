
function list_datastores() {
    satori list --datastores | while IFS= read -r line; do
        echo "'$line'"
    done
}

function list_databases() {
    local datastore_name=$(echo -n $1 | sed "s/'//g")
    satori list --databases "$datastore_name"
}

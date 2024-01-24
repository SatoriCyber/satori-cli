

_datastores() {
    local datastore_names=()

    # Read keys line by line and populate the array
    while IFS= read -r line; do
        datastore_names+=("$line")
    done < <(satori list --datastores)

    _values datastore_name $datastore_names
}


_databases() {
    local datastore_name="$words[CURRENT-1]"  # Get the selected datastore_name
    datastore_name=${datastore_name//\\/}
    local databases=()
    while IFS= read -r line; do
        databases+=("$line")
    done < <(satori list --databases $datastore_name)
    if [[ ! -z "$databases" ]]; then
        _values database $databases
    fi
}
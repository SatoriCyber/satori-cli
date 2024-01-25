function Get-CompletionDatastores {
    $additionalResults = @()
    $datastoreResults = Invoke-Expression 'satori list --datastores'
    $datastoreResults -split '\r?\n' | ForEach-Object {
        $additionalResults += [CompletionResult]::new("'$_'", "'$_'", [CompletionResultType]::ParameterValue, "Datastore: $_")
    }
    return $additionalResults
}
 
function Get-CompletionDatabases {
    param (
        [string]$DatastoreName
    )
        $databases = @()
        $listOfdatabases = Invoke-Expression "satori list --databases $datastoreName"
        $listOfdatabases -split '\r?\n' | ForEach-Object {
            $databases += [CompletionResult]::new($_, $_, [CompletionResultType]::ParameterValue, "Database: $_")
        }
        return $databases
}


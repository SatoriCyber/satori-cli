
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'satori' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'satori'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'satori' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('run', 'run', [CompletionResultType]::ParameterValue, 'Execute to a tool')
            [CompletionResult]::new('login', 'login', [CompletionResultType]::ParameterValue, 'Login to Satori')
            [CompletionResult]::new('auto_complete', 'auto_complete', [CompletionResultType]::ParameterValue, 'Generate autocomplete')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List resources')
            [CompletionResult]::new('pgpass', 'pgpass', [CompletionResultType]::ParameterValue, 'Creates a Pgpass file to be used by other apps')
            [CompletionResult]::new('aws', 'aws', [CompletionResultType]::ParameterValue, 'Creates a aws profile to be used with s3')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'satori;run' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('dbt', 'dbt', [CompletionResultType]::ParameterValue, 'dbt')
            [CompletionResult]::new('psql', 'psql', [CompletionResultType]::ParameterValue, 'psql')
            [CompletionResult]::new('mongosh', 'mongosh', [CompletionResultType]::ParameterValue, 'mongosh')
            [CompletionResult]::new('s3', 's3', [CompletionResultType]::ParameterValue, 's3')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'satori;run;dbt' {
            [CompletionResult]::new('--profile-dir', 'profile-dir', [CompletionResultType]::ParameterName, 'The path to the dbt profiles directory')
            [CompletionResult]::new('--target', 'target', [CompletionResultType]::ParameterName, 'DBT target')
            [CompletionResult]::new('--domain', 'domain', [CompletionResultType]::ParameterName, 'INTERNAL Default to https://app.satoricyber.com')
            [CompletionResult]::new('--no-launch-browser', 'no-launch-browser', [CompletionResultType]::ParameterName, 'Don''t launch the browser')
            [CompletionResult]::new('--invalid_cert', 'invalid_cert', [CompletionResultType]::ParameterName, 'INTERNAL disable SSL verification')
            [CompletionResult]::new('--debug', 'debug', [CompletionResultType]::ParameterName, 'Enable debug mode')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'refresh the local cache files')
            [CompletionResult]::new('--no-persist', 'no-persist', [CompletionResultType]::ParameterName, 'Don''t persist the database credentials')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'satori;run;psql' {
            $completionResults += Get-CompletionDatastores
            $completionResults
        break
        }
        'satori;run;mongosh' {
            $completionResults += Get-CompletionDatastores
            $completionResults
        break
        }
        'satori;run;s3' {
            $completionResults += Get-CompletionDatastores
            $completionResults
        break
        }
        'satori;run;help' {
            [CompletionResult]::new('dbt', 'dbt', [CompletionResultType]::ParameterValue, 'dbt')
            [CompletionResult]::new('psql', 'psql', [CompletionResultType]::ParameterValue, 'psql')
            [CompletionResult]::new('mongosh', 'mongosh', [CompletionResultType]::ParameterValue, 'mongosh')
            [CompletionResult]::new('s3', 's3', [CompletionResultType]::ParameterValue, 's3')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'satori;run;help;dbt' {
            break
        }
        'satori;run;help;psql' {
            break
        }
        'satori;run;help;mongosh' {
            break
        }
        'satori;run;help;s3' {
            break
        }
        'satori;run;help;help' {
            break
        }
        'satori;login' {
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'format')
            [CompletionResult]::new('--domain', 'domain', [CompletionResultType]::ParameterName, 'INTERNAL Default to https://app.satoricyber.com')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Display the credentials or save to file')
            [CompletionResult]::new('--display', 'display', [CompletionResultType]::ParameterName, 'Display the credentials or save to file')
            [CompletionResult]::new('--no-launch-browser', 'no-launch-browser', [CompletionResultType]::ParameterName, 'Don''t launch the browser')
            [CompletionResult]::new('--invalid_cert', 'invalid_cert', [CompletionResultType]::ParameterName, 'INTERNAL disable SSL verification')
            [CompletionResult]::new('--debug', 'debug', [CompletionResultType]::ParameterName, 'Enable debug mode')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'refresh the local cache files')
            [CompletionResult]::new('--no-persist', 'no-persist', [CompletionResultType]::ParameterName, 'Don''t persist the database credentials')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'satori;auto_complete' {
            [CompletionResult]::new('--generate', 'generate', [CompletionResultType]::ParameterName, 'Generate completion file')
            [CompletionResult]::new('--out', 'out', [CompletionResultType]::ParameterName, 'Output file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'satori;list' {
            [CompletionResult]::new('--databases', 'databases', [CompletionResultType]::ParameterName, 'List of databases for the datastore')
            [CompletionResult]::new('--datastores', 'datastores', [CompletionResultType]::ParameterName, 'Get all available datastores')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'satori;pgpass' {
            [CompletionResult]::new('--domain', 'domain', [CompletionResultType]::ParameterName, 'INTERNAL Default to https://app.satoricyber.com')
            [CompletionResult]::new('--no-launch-browser', 'no-launch-browser', [CompletionResultType]::ParameterName, 'Don''t launch the browser')
            [CompletionResult]::new('--invalid_cert', 'invalid_cert', [CompletionResultType]::ParameterName, 'INTERNAL disable SSL verification')
            [CompletionResult]::new('--debug', 'debug', [CompletionResultType]::ParameterName, 'Enable debug mode')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'refresh the local cache files')
            [CompletionResult]::new('--no-persist', 'no-persist', [CompletionResultType]::ParameterName, 'Don''t persist the database credentials')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'satori;aws' {
            [CompletionResult]::new('--domain', 'domain', [CompletionResultType]::ParameterName, 'INTERNAL Default to https://app.satoricyber.com')
            [CompletionResult]::new('--no-launch-browser', 'no-launch-browser', [CompletionResultType]::ParameterName, 'Don''t launch the browser')
            [CompletionResult]::new('--invalid_cert', 'invalid_cert', [CompletionResultType]::ParameterName, 'INTERNAL disable SSL verification')
            [CompletionResult]::new('--debug', 'debug', [CompletionResultType]::ParameterName, 'Enable debug mode')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'refresh the local cache files')
            [CompletionResult]::new('--no-persist', 'no-persist', [CompletionResultType]::ParameterName, 'Don''t persist the database credentials')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'satori;help' {
            [CompletionResult]::new('run', 'run', [CompletionResultType]::ParameterValue, 'Execute to a tool')
            [CompletionResult]::new('login', 'login', [CompletionResultType]::ParameterValue, 'Login to Satori')
            [CompletionResult]::new('auto_complete', 'auto_complete', [CompletionResultType]::ParameterValue, 'Generate autocomplete')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List resources')
            [CompletionResult]::new('pgpass', 'pgpass', [CompletionResultType]::ParameterValue, 'Creates a Pgpass file to be used by other apps')
            [CompletionResult]::new('aws', 'aws', [CompletionResultType]::ParameterValue, 'Creates a aws profile to be used with s3')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'satori;help;run' {
            [CompletionResult]::new('dbt', 'dbt', [CompletionResultType]::ParameterValue, 'dbt')
            [CompletionResult]::new('psql', 'psql', [CompletionResultType]::ParameterValue, 'psql')
            [CompletionResult]::new('mongosh', 'mongosh', [CompletionResultType]::ParameterValue, 'mongosh')
            [CompletionResult]::new('s3', 's3', [CompletionResultType]::ParameterValue, 's3')
            break
        }
        'satori;help;run;dbt' {
            break
        }
        'satori;help;run;psql' {
            break
        }
        'satori;help;run;mongosh' {
            break
        }
        'satori;help;run;s3' {
            break
        }
        'satori;help;login' {
            break
        }
        'satori;help;auto_complete' {
            break
        }
        'satori;help;list' {
            break
        }
        'satori;help;pgpass' {
            break
        }
        'satori;help;aws' {
            break
        }
        'satori;help;help' {
            break
        }
    })

    
    if ($completions.Count -eq 0) {
        
            if($command.StartsWith('satori;run;psql')) {
                if ($commandElements.Count -eq 4) {
                    $datastoreName = $commandElements[3]
                    $completions = Get-CompletionDatabases -DatastoreName $datastoreName
                
                } else {
                    $completions=@(
            [CompletionResult]::new('--domain', 'domain', [CompletionResultType]::ParameterName, 'INTERNAL Default to https://app.satoricyber.com')
            [CompletionResult]::new('--no-launch-browser', 'no-launch-browser', [CompletionResultType]::ParameterName, 'Don''t launch the browser')
            [CompletionResult]::new('--invalid_cert', 'invalid_cert', [CompletionResultType]::ParameterName, 'INTERNAL disable SSL verification')
            [CompletionResult]::new('--debug', 'debug', [CompletionResultType]::ParameterName, 'Enable debug mode')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'refresh the local cache files')
            [CompletionResult]::new('--no-persist', 'no-persist', [CompletionResultType]::ParameterName, 'Don''t persist the database credentials')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            )
                }
            }
            
            if($command.StartsWith('satori;run;mongosh')) {
                $completions=@(
            [CompletionResult]::new('--domain', 'domain', [CompletionResultType]::ParameterName, 'INTERNAL Default to https://app.satoricyber.com')
            [CompletionResult]::new('--no-launch-browser', 'no-launch-browser', [CompletionResultType]::ParameterName, 'Don''t launch the browser')
            [CompletionResult]::new('--invalid_cert', 'invalid_cert', [CompletionResultType]::ParameterName, 'INTERNAL disable SSL verification')
            [CompletionResult]::new('--debug', 'debug', [CompletionResultType]::ParameterName, 'Enable debug mode')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'refresh the local cache files')
            [CompletionResult]::new('--no-persist', 'no-persist', [CompletionResultType]::ParameterName, 'Don''t persist the database credentials')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            )
            }
            
            if($command.StartsWith('satori;run;s3')) {
                $completions=@(
            [CompletionResult]::new('--domain', 'domain', [CompletionResultType]::ParameterName, 'INTERNAL Default to https://app.satoricyber.com')
            [CompletionResult]::new('--no-launch-browser', 'no-launch-browser', [CompletionResultType]::ParameterName, 'Don''t launch the browser')
            [CompletionResult]::new('--invalid_cert', 'invalid_cert', [CompletionResultType]::ParameterName, 'INTERNAL disable SSL verification')
            [CompletionResult]::new('--debug', 'debug', [CompletionResultType]::ParameterName, 'Enable debug mode')
            [CompletionResult]::new('--refresh', 'refresh', [CompletionResultType]::ParameterName, 'refresh the local cache files')
            [CompletionResult]::new('--no-persist', 'no-persist', [CompletionResultType]::ParameterName, 'Don''t persist the database credentials')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            )
            }
            
    
    }

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
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


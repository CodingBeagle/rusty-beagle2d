Param (
    [Parameter(Mandatory=$true)]
    [String]
    $WorkspaceRoot,
    [Parameter(Mandatory=$true)]
    [String]
    $DestinationPath
)

# Copy GLFW DLL to target directory
Copy-Item -Path "$WorkspaceRoot/rusty-beagle2d-glfw/libs/glfw/glfw3.dll" -Destination $DestinationPath

# Copy dat folder to target directory
Copy-Item -Path "$WorkspaceRoot/dat" -Destination $DestinationPath -Recurse -Force -Verbose
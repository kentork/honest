Param( [string] $str )

[string]::concat(
  (
    [security.cryptography.SHA256]::create().computehash(
      [text.encoding]::ascii.getbytes($str)
    ) | % { $_.tostring("x2") }
  )
)

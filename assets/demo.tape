Output demo.gif

Require chess-cat

Set Shell "zsh"
Set FontSize 22
Set FontFamily "Cascadia Code NF"
Set Width 1400
Set Height 900
Set Theme "ayu"
Set TypingSpeed 20ms

# Show help
Type 'chess-cat --help' Sleep 500ms Enter
Sleep 2s

# Starting position (default classic theme)
Type 'chess-cat' Sleep 500ms Enter
Sleep 2s

# Italian Game opening — nice developed position
Type 'chess-cat "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4"' Sleep 500ms Enter
Sleep 2s

# Green theme (popular chess.com style)
Type 'chess-cat --theme green "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4"' Sleep 500ms Enter
Sleep 2s

# Ocean theme
Type 'chess-cat --theme ocean "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4"' Sleep 500ms Enter
Sleep 2s

# Flip — show from Black's perspective
Type 'chess-cat --flip "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4"' Sleep 500ms Enter
Sleep 2s

# Duck chess!
Type 'echo "chess-cat also supports Duck chess FENs"' Enter
Sleep 250ms
Type 'chess-cat "8/8/1p2k1p1/3p2*p/1p1P1P1P/1P2PK2/8/8 w - - 3 54"' Sleep 500ms Enter
Sleep 2s

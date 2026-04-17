Output demo.gif

Require chess-cat

Set Shell "zsh"
Set FontSize 22
Set FontFamily "Cascadia Code NF"
Set Width 1400
Set Height 900
Set Theme "ayu"
Set TypingSpeed 40ms

# Show help
Type 'chess-cat --help' Sleep 500ms Enter
Sleep 3s

# Starting position (default classic theme)
Type 'chess-cat' Sleep 500ms Enter
Sleep 3s

# Italian Game opening — nice developed position
Type 'chess-cat "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4"' Sleep 500ms Enter
Sleep 3s

# Green theme (popular chess.com style)
Type 'chess-cat --theme green "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4"' Sleep 500ms Enter
Sleep 3s

# Ocean theme
Type 'chess-cat --theme ocean "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4"' Sleep 500ms Enter
Sleep 3s

# Flip — show from Black's perspective
Type 'chess-cat --flip "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4"' Sleep 500ms Enter
Sleep 3s

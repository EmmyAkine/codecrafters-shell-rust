Now, I kinda need your help, you helped me write tokenizer the last time for the c# version. I'm back in that area now.

This is what you gave me then that covered Unix and windows


```
public enum TokenKind
{
    Word,           // regular argument
    RedirectOut,    // >  or 1>   (stdout, truncate)
    RedirectAppend, // >> or 1>>  (stdout, append)
    RedirectErr,    // 2>         (stderr, truncate)
    RedirectErrAppend, // 2>>     (stderr, append)
    Pipe,
}

public record Token(TokenKind Kind, string Value);

```

So, I need you to do help me write the rust version, I don't fair well when it comes to tokenizing, string parsing logic or anything related to regex....lol😂😂😂


***
**The Tokenizer Class**

````

/// <summary>
/// Tokenizes shell input into a list of Tokens, handling:
///   - single quotes      : everything literal, no escaping
///   - double quotes      : only \" and \\ are escape sequences
///   - unquoted backslash : escapes any character
///   - spaces             : split tokens
///   - redirect operators : > >> 1> 1>> 2> 2>>
/// </summary>
public class Tokenizer
{
    private string _input = "";
    private int _pos;
    private readonly List<Token> _tokens = [];
    private readonly System.Text.StringBuilder _current = new();

    public static List<Token> Tokenize(string input)
    {
        var t = new Tokenizer { _input = input, _pos = 0 };
        return t.Run();
    }

    private List<Token> Run()
    {
        while (_pos < _input.Length)
        {
            char c = _input[_pos];

            if      (c == '\'')              ReadSingleQuoted();
            else if (c == '"')               ReadDoubleQuoted();
            else if (c == '\\')              ReadUnquotedEscape();
            else if (c == ' ')               SkipSpaceAndFlush();
            else if (IsRedirectStart(c))     ReadRedirect();
            else if (c == '|')               ReadPipe();
            else                             Consume();
        }

        // flush any trailing word
        if (_current.Length > 0)
            _tokens.Add(new Token(TokenKind.Word, _current.ToString()));

        return _tokens;
    }

    // ── single-quoted: everything literal until closing ' ────────────────────
    private void ReadSingleQuoted()
    {
        _pos++;  // skip opening '
        while (_pos < _input.Length && _input[_pos] != '\'')
            _current.Append(_input[_pos++]);

        if (_pos >= _input.Length)
            throw new InvalidOperationException("syntax error: unterminated single quote");

        _pos++;  // skip closing '
    }

    // ── double-quoted: \" and \\ are escapes, everything else literal ─────────
    private void ReadDoubleQuoted()
    {
        _pos++;  // skip opening "
        while (_pos < _input.Length && _input[_pos] != '"')
        {
            if (_input[_pos] == '\\' && _pos + 1 < _input.Length)
            {
                char next = _input[_pos + 1];
                if (next == '"' || next == '\\')
                {
                    _pos++;
                    _current.Append(_input[_pos++]);
                    continue;
                }
            }
            _current.Append(_input[_pos++]);
        }

        if (_pos >= _input.Length)
            throw new InvalidOperationException("syntax error: unterminated double quote");

        _pos++;  // skip closing "
    }

    // ── unquoted backslash: next char is always literal ──────────────────────
    private void ReadUnquotedEscape()
    {
        _pos++;  // skip backslash
        if (_pos >= _input.Length)
            throw new InvalidOperationException("syntax error: backslash at end of input");

        _current.Append(_input[_pos++]);
    }

    // ── space: flush current word (if any) and advance past the space ─────────
    private void SkipSpaceAndFlush()
    {
        if (_current.Length > 0)
        {
            _tokens.Add(new Token(TokenKind.Word, _current.ToString()));
            _current.Clear();
        }
        _pos++;  // skip the space
    }

    // ── redirect operators: > >> 1> 1>> 2> 2>> ───────────────────────────────
    private bool IsRedirectStart(char c)
    {
        if (c == '>') return true;
        if ((c == '1' || c == '2') && _pos + 1 < _input.Length && _input[_pos + 1] == '>')
            return true;
        return false;
    }

    private void ReadRedirect()
    {
        // flush any word built up so far WITHOUT advancing _pos (no space to skip)
        if (_current.Length > 0)
        {
            _tokens.Add(new Token(TokenKind.Word, _current.ToString()));
            _current.Clear();
        }

        string fd = "";
        if (_input[_pos] == '1' || _input[_pos] == '2')
            fd = _input[_pos++].ToString();  // consume "1" or "2"

        _pos++;  // consume '>'

        bool append = _pos < _input.Length && _input[_pos] == '>';
        if (append) _pos++;  // consume second '>'

        var kind = (fd, append) switch
        {
            ("2", true)  => TokenKind.RedirectErrAppend,
            ("2", false) => TokenKind.RedirectErr,
            (_,   true)  => TokenKind.RedirectAppend,
            (_,   false) => TokenKind.RedirectOut,
        };

        _tokens.Add(new Token(kind, ""));
    }

    private void ReadPipe() {
        if (_current.Length > 0) {
            _tokens.Add(new Token(TokenKind.Word, _current.ToString()));
            _current.Clear();
        }
        _tokens.Add(new Token(TokenKind.Pipe, ""));
        _pos++;
    }

    // ── normal character ─────────────────────────────────────────────────────
    private void Consume() => _current.Append(_input[_pos++]);
}

````
program TestAllTokens;

{ This is a block comment
  Good luck UTS. }

var
  my_integer, another_var : integer;
  a_real_number         : real;
  is_done               : boolean;
  my_char               : char;

const
  PI = 3.14159;

begin
  (* Tes assignments and expressions *)
  my_integer := 100;
  another_var := my_integer + 20 div 5;
  a_real_number := my_integer / 3.0;

  (* Tes Relational and logical operators *)
  if (my_integer > 50) and (another_var <> 104) then
  begin
    is_done := true;
  end
  else
  begin
    is_done := false;
  end;

  (* Character and String Literals *)
  my_char := 'A';
  writeln('This is a test string literal.');

  (* Testing multi-character operators *)
  if another_var <= 105 then
    writeln('Less than or equal');

  (* Range operator for arrays or subranges *)
  { array declaration is just for tokenizing '..' }
  type
    Numbers = array[1..10] of integer;

end. (* End of the test program. *)

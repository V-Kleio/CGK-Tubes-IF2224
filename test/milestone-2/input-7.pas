program TestAllTokens;

{ This is a block comment
  Good luck UTS. }

variabel
  my_integer, another_var : integer;
  a_real_number         : real;
  is_done               : boolean;
  my_char               : char;

konstanta
  PI = 3.14159;

(* Range operator for arrays or subranges *)
{ array declaration is just for tokenizing '..' }
tipe
  Numbers = larik[1..10] dari integer;

mulai
  (* Tes assignments and expressions *)
  my_integer := 100;
  another_var := my_integer + 20;
  a_real_number := my_integer / 3.0;

  (* Tes Relational and logical operators *)
  jika (my_integer > 50) dan (another_var <> 104) maka
  mulai
    is_done := true;
  selesai
  selain_itu
  mulai
    is_done := false;
  selesai;

  (* Character and String Literals *)
  my_char := 'A';
  writeln('This is a test string literal.');

  (* Testing multi-character operators *)
  jika another_var <= 105 maka
    writeln('Less than or equal');

selesai. (* End of the test program. *)

program NestedTest;
variabel x: integer;

prosedur Outer;
  
  prosedur Inner;
    mulai
      x := 10;
      writeln(x)
    selesai;
  
  mulai
    Inner;
    writeln('Done')
  selesai;

mulai
  x := 0;
  Outer
selesai.
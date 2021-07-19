###################
Palarel programming
###################

マンデルプロ集合
===================


Usage:
------------

:: 

  mandelbrot Filename PIXELS UPPERLEFT LOWERRIGHT
  Example: cargo run mandel.png 1000x750 -1.20,0.35 -1,0.20


.. image:: mandel.png
    :alt: 実行結果


thread版
----------

::
  
  ❯ time target/release/mandelbrot mandel-thread.png 4000x3000 -1.20,0.35 -1.0,0.20

  real    0m1.491s
  user    0m4.169s
  sys     0m0.028s-

.. image:: mandel-thread.png
   :alt: 実行結果



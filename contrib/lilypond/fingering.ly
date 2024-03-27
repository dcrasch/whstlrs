\language "english"

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%
% Usage example:
%
%  \new NoteNames {
%    %#(define myFingSize 0.75)  % this line would override the size of the diagram produced
%    %#(define myFingThickness  0.25)  % this line would override the thickness of the lines used to draw the diagrams
%    \set noteNameFunction = #myTWFingerings
%    \music
%  }
%
%  \new NoteNames {
%    \set noteNameFunction = #myTWNoteNames
%    \music
%  }
%
% Both can be used to put the note name under (or over, if you prefer) the fingering diagram.
% Just put one after the other in the prefered order. Just make sure to enclose them in
% a set of angle brackets for simultaneous music (i.e. put them in a << >> set).


#(define myFingSize  0.65)  %  Overall size of the fingering diagram
#(define myFingThickness  0.25)  %  How thick the lines used to draw the diagram are


myTWFingerings =           
#(lambda (pitch ctx)
   (cond
    ((<= (ly:pitch-semitones pitch)
         1)  ; Note too low
     (markup #:sans "U"))
    ((>= (ly:pitch-semitones pitch)
         27)  ;  Note too high
     (markup #:sans "O"))

    ; D4
    ((= (ly:pitch-semitones pitch)
        2)
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two 'three 'four 'five 'six)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; D#/Eb4 or 5
    ((or (= (ly:pitch-semitones pitch)
            3)
         (= (ly:pitch-semitones pitch)
            15))
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two 'three 'four 'five 'six1h)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; E4 or 5
    ((or (= (ly:pitch-semitones pitch)
            4)
         (= (ly:pitch-semitones pitch)
            16))
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two 'three 'four 'five)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; F4 or 5
    ((or (= (ly:pitch-semitones pitch)
            5)
         (= (ly:pitch-semitones pitch)
            17))
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two 'three 'four 'five1h)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; F# or Gb4 or 5
    ((or (= (ly:pitch-semitones pitch)
            6)
         (= (ly:pitch-semitones pitch)
            18))
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two 'three 'four)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; G4 or 5
    ((or (= (ly:pitch-semitones pitch)
            7)
         (= (ly:pitch-semitones pitch)
            19))
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two 'three)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; G# or Ab4
    ((= (ly:pitch-semitones pitch)
        8)
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two 'three1h)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; G# or Ab5
    ((= (ly:pitch-semitones pitch)
        20)
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two 'four 'five)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; A4 or 5
    ((or (= (ly:pitch-semitones pitch)
            9)
         (= (ly:pitch-semitones pitch)
            21))
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; A# or Bb4
    ((= (ly:pitch-semitones pitch)
        10)
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two1h)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; A# or Bb5
    ((= (ly:pitch-semitones pitch)
        22)
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one 'two1h)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; B4 or 5
    ((or (= (ly:pitch-semitones pitch)
            11)
         (= (ly:pitch-semitones pitch)
            23))
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; C5 or D6
    ((or (= (ly:pitch-semitones pitch)
            12)
         (= (ly:pitch-semitones pitch)
            26))
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'two 'three)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; C# or Db5 or 6
    ((or (= (ly:pitch-semitones pitch)
            13)
         (= (ly:pitch-semitones pitch)
            25))
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc )
                                      (list 'lh)
                                      (list 'rh))))))))

    ; C6
    ((= (ly:pitch-semitones pitch)
        24)
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'one1h)
                                      (list 'lh)
                                      (list 'rh))))))))

    ; D5
    ((= (ly:pitch-semitones pitch)
        14)
     (markup
      (#:override `(size . ,myFingSize)
                  (#:override `(thickness . ,myFingThickness)
                              (#:center-column
                               (#:pad-around
                                0.15
                                #:woodwind-diagram
                                'tin-whistle
                                (list (list 'cc 'two 'three 'four 'five 'six)
                                      (list 'lh)
                                      (list 'rh))))))))

    (else  ;  Failover - should never get here
           (markup #:sans "X"))

    )
   )


myTWNoteNames = 
#(lambda (pitch ctx)
   (cond
    ((<= (ly:pitch-semitones pitch)
         1) ; Note too low
     (markup #:sans "U"))
    ((>= (ly:pitch-semitones pitch)
         27) ; Note too high
     (markup #:sans "O"))
    ((= (ly:pitch-semitones pitch)
        26) ; D6
     (markup #:sans (#:center-column
                     (#:pad-around 0.6
                                   (note-name->markup pitch #t)
                                   "++"))))
    (else
     (if (<= (ly:pitch-semitones pitch)
             13)
         (markup #:sans (note-name->markup pitch #f))
         (markup #:sans (#:center-column
                         (#:pad-around 0.6
                                       (note-name->markup pitch #t)
                                       "+")))
         )
     )
    )
   )

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%
% Example code begins
%

music = {
  \key d \major
  cs'4 d' e' f' fs' g' a' b' c'' cs'' d'' e''
  f'' fs'' g'' a'' b'' c''' cs''' d''' e'''
}

\score {
  <<
    
    \new Staff {
      \new Voice {
        \music
      }
    }
    
    %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
    % First NoteNames context used to produce the fingering diagrams
    \new NoteNames {
      %#(define myFingSize 0.75)  % this line would override the size of the diagram produced
      %#(define myFingThickness  0.25)  % this line would override the thickness of the lines used to draw the diagrams
      \set noteNameFunction = #myTWFingerings
      \music
    }

    %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
    % Second NoteNames context used to put the note names
    % below the fingering diagrams
    \new NoteNames {
      \set noteNameFunction = #myTWNoteNames
      \music
    }
  
  >>
}

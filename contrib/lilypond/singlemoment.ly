\version "2.24"


\header {
  crossRefNumber = "1"
  footnotes = ""
  meter = "jig"
  tagline = "Lily was here 2.24.1 -- automatically converted from ABC"
  title = "Star Of The County Down"
}

#(ly:set-option 'backend 'svg)
\pointAndClickOff
#(define (format-moment moment)
  (exact->inexact
   (/ (ly:moment-main-numerator moment)
    (ly:moment-main-denominator moment))))

#(define (attrs-notehead grob)
  (let* ((origin (ly:input-file-line-char-column
		  (ly:event-property (ly:grob-property grob 'cause) 'origin)))
	 (duration (ly:event-property (ly:grob-property grob 'cause) 'duration))
	 (pitch (ly:event-property (ly:grob-property grob 'cause) 'pitch))
       )
   (list
    (cons 'class "note-head")
    (cons 'id (ly:format "Note-~a-~a" (cadr origin) (caddr origin)))
    (cons 'data-duration (ly:format "~a" (format-moment (ly:duration-length duration))))
    (cons 'data-pitch (+ 60 (ly:pitch-semitones pitch)))
)))


#(define (attrs-notehead-after grob grob-origin context)
  (let (
    (moment (ly:context-current-moment context))
    )
     (set! (ly:grob-property grob 'output-attributes) (list
			       (cons 'class "note-head")
			       (cons 'data-moment (ly:format "~a" (format-moment moment)))
			     ))))


voicedefault =  {
  \key e \minor   b8[    d'8]  \bar "|"   d'8[    e'8    e'8]    e'8[   
    d'8    e'8]  \bar "|"   g'8[    e'8    g'8]    a'8[    g'8    a'8]  \bar "|"   
  b'8[    a'8    g'8]    e'8[    d'8    b8]  \bar "|"   d'8[    e'8    d'8]    
  d'8[    b8    d'8]  \bar "|"     d'8[    e'8    e'8]    e'8[    d'8    e'8]  
  \bar "|"   g'8[    e'8    g'8]    a'8[    g'8    a'8]  \bar "|"   b'8[    a'8   
									    g'8]    e'8[    g'8    d'8]  \bar "|"   d'8[    e'8    e'8]    e'8[    g'8    
																		   b'8]  \bar "||"     d''8[    b'8    b'8]    b'8[    a'8    g'8]  \bar "|"   a'8 
  [   b'8    a'8]    a'8[    g'8    a'8]  \bar "|"   b'8[    a'8    g'8]    e'8[  
    d'8    b8]  \bar "|"   d'8[    e'8    d'8]    d'8[    b8    d'8]  \bar "|"    
  d'8[    e'8    e'8]    e'8[    d'8    e'8]  \bar "|"   g'8[    e'8    g'8]    
  a'8[    g'8    a'8]  \bar "|"   b'8[    a'8    g'8]    e'8[    g'8    d'8]  
  \bar "|"   d'8[    e'8    e'8]    e'8  \bar "||"   
}

\score{
  <<

    \context Staff="default"
    {
      \override NoteHead.output-attributes = #attrs-notehead      
      %\applyOutput Voice.NoteHead #attrs-notehead-after
      \voicedefault
    }
  >>
  \layout {
    
  }
  \midi {}
 
}

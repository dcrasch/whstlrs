\version "2.24"


\header {
	crossRefNumber = "1"
	footnotes = ""
	meter = "jig"
	tagline = "Lily was here 2.24.1 -- automatically converted from ABC"
	title = "Star Of The County Down"
      }

\include "event-listener.ly"
#(ly:set-option 'backend 'svg)
\pointAndClickOff

#(define (attrs-notehead grob)
     (let* ((origin (ly:input-file-line-char-column
		     (ly:event-property (ly:grob-property grob 'cause) 'origin))))
      (list (cons 'class "note-head")
       (cons
	     'id   (string-concatenate 
       (list 
         "Note-" 
         (ly:format "~a-~a"
           (cadr origin)
	  (caddr origin))))))))


voicedefault =  {
\set Score.measureBarType = ""

\time 6/8 \key e \minor   b8[    d'8]  \bar "|"   d'8[    e'8    e'8]    e'8[   
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
	  \override NoteHead.output-attributes  = #attrs-notehead


	    \voicedefault 
	}

    >>
	\layout {
	}
	\midi {}
}

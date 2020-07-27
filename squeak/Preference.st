'From Cuis 5.0 [latest update: #4191] on 14 June 2020 at 12:45:47 pm'!
!classDefinition: #Preference category: #'System-Support'!
Object subclass: #Preference
	instanceVariableNames: 'name value defaultValue helpString categoryList changeInformee changeSelector'
	classVariableNames: ''
	poolDictionaries: ''
	category: 'System-Support'!
!Preference commentStamp: 'sw 1/14/2003 22:33' prior: 0!
Represents a true/false flag that is under user control and which can be interrogated by a call to Preferences

	name 				a symbol, the formal name of the preference.
	value				a boolean, the current value
	defaultValue		the default value of the preference
	helpString 			string or text, constituting the help message
	localToProject		boolean, whether each project holds its own version
	categoryList			list of categories under which to offer this
	changeInformee 	whom, if anyone, to inform if the value changes:
	changeSelector 		what selector to send to the changeInformee when the value changes!


!Preference methodsFor: 'initialization' stamp: 'jmv 9/2/2013 09:04'!
name: aName defaultValue: aValue helpString: aString categoryList: aList changeInformee: informee changeSelector:  aChangeSelector
	"Initialize the preference from the given values.  There is an extra tolerence here for the symbols #true, #false, and #nil, which are interpreted, when appropriate, as meaning true, false, and nil"

	name _ aName asSymbol.
	defaultValue _ aValue.
	defaultValue = #true ifTrue: [ defaultValue _ true ].
	defaultValue = #false ifTrue: [ defaultValue _ false ].
	value _ defaultValue.
	helpString _ aString.
	categoryList _ (aList ifNil: [OrderedCollection with: #unclassified]) collect:
		[:elem | elem asSymbol].

	changeInformee _ (informee == nil or: [informee == #nil])
						ifTrue: [nil]
						ifFalse:	[(informee isSymbol)
							ifTrue: [
								Smalltalk at: informee]
							ifFalse: [
								informee]].
	changeSelector  _ aChangeSelector! !


!Preference methodsFor: 'menu' stamp: 'sw 4/10/2001 15:02'!
name
	"Answer this preference's name"

	^ name! !


!Preference methodsFor: 'value' stamp: 'sw 4/10/2001 15:01'!
defaultValue
	"Answer this preference's defaultValue"

	^ defaultValue! !

!Preference methodsFor: 'value' stamp: 'sw 4/10/2001 15:35'!
preferenceValue
	"Answer the current value of the preference"

	^ value! !

!Preference methodsFor: 'value' stamp: 'sw 4/12/2001 23:28'!
preferenceValue: aValue
	"set the value as indicated, and invoke the change selector if appropriate"

	| oldValue |
	oldValue _ value.
	value _ aValue.
	oldValue ~~ value ifTrue:
		[self notifyInformeeOfChange]! !

!Preference methodsFor: 'value' stamp: 'sw 4/12/2001 00:04'!
restoreDefaultValue
	"restore the default value to the preference"

	value _ defaultValue! !

!Preference methodsFor: 'value' stamp: 'sw 4/12/2001 00:04'!
togglePreferenceValue
	"Toggle whether the value of the preference"

	value _ value not.
	self notifyInformeeOfChange! !


!Preference methodsFor: 'change notification' stamp: 'sw 4/12/2001 01:39'!
changeInformee: informee changeSelector: aSelector
	"Set the changeInformee and changeSelector as specified"

	changeInformee _ informee.
	changeSelector _ aSelector! !

!Preference methodsFor: 'change notification' stamp: 'sw 4/12/2001 00:03'!
notifyInformeeOfChange
	"If there is a changeInformee, notify her that I have changed value"

	changeInformee ifNotNil: [changeInformee perform: changeSelector]! !


!Preference methodsFor: 'debugging' stamp: 'sw 4/13/2001 00:05'!
printOn: aStream
	"Print a string decribing the receiver to the given stream"

	super printOn: aStream.
	aStream nextPutAll: name storeString, ' ', value storeString! !

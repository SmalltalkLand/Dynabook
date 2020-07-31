'From Cuis 5.0 [latest update: #4257] on 29 July 2020 at 9:03:17 pm'!

!Preferences class methodsFor: 'bigger and smaller GUI' stamp: 'KLG 7/29/2020 20:56:57'!
haloHandleScale
	^ self
		preferenceAt: #haloHandleScale
		ifAbsent: [ 1 ].! !

!Preferences class methodsFor: 'bigger and smaller GUI' stamp: 'KLG 7/29/2020 20:52:47'!
haloHandleScale: aScale
	"Set the scaling factor for the system window border width."

	HaloMorph releaseClassCachedState.
	^ self 
		setPreference: #haloHandleScale
		toValue: aScale asInteger! !


!HaloMorph methodsFor: 'private' stamp: 'KLG 7/29/2020 21:00:35'!
addHandle: handleSpec
	"Add a handle within the halo box as per the haloSpec, and set it up to respond to the given event by sending the given selector to the given recipient.  Return the handle."

	| handle aPoint colorToUse form icon |
	aPoint _ self 
				positionIn: haloBox
				horizontalPlacement: handleSpec horizontalPlacement
				verticalPlacement: handleSpec verticalPlacement.
	colorToUse _ Color colorFrom: handleSpec color.
	handle _ HaloHandleMorph new color: colorToUse.
	self addMorph: handle.
	handle morphBoundsInWorld: (Rectangle center: aPoint extent: self class handleSize asPoint).
	handleSpec iconSymbol ifNotNil: [ :iconName |
			form _ self class icons at: iconName ifAbsent: [self class perform: iconName].
			form ifNotNil: [
				| handleSize |
				handleSize _ self class handleSize.
				icon _ ImageMorph new
					image: ((handleSize = 16) ifFalse: [  ": Non default size, scale that bugger!!"
						form  ": Be as smooth as possible, these images are small."
							magnify: form boundingBox
							to: (handleSize @ handleSize)
							smoothing: 1 ] ifTrue: [ form ]);
					color: colorToUse makeForegroundColor;
					lock.
				handle addMorphFront: icon position: `0@0` ]].
	handle mouseUpSelector: #endInteraction.
	handle setBalloonText: handleSpec hoverHelp.
	^ handle! !


!HaloMorph class methodsFor: 'cached state access' stamp: 'KLG 7/29/2020 20:53:19'!
handleSize
	HandleSize ifNil: [
		HandleSize _ 16 * Preferences haloHandleScale ].
	^ HandleSize! !


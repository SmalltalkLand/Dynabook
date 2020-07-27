'From Cuis 5.0 [latest update: #4249] on 5 July 2020 at 4:48:48 pm'!

!StringTest methodsFor: 'tests - grammar' stamp: 'LC 7/5/2020 16:47:10'!
testArticle
	self
		assert: 'euphemism' withArticle equals: 'a euphemism';
		assert: 'European' withArticle equals: 'a European';
		assert: 'Euclidean space' withArticle equals: 'a Euclidean space';
		assert: 'university' withArticle equals: 'a university';
		assert: 'anagram' withArticle equals: 'an anagram';
		assert: 'apple' withArticle equals: 'an apple';
		assert: 'Ukranian' withArticle equals: 'a Ukranian';
		assert: 'Argentine' withArticle equals: 'an Argentine';
		assert: 'user' withArticle equals: 'a user'! !


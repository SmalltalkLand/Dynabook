extends RigidBody


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var xmovement = 0
	var ymovement = 0
	if Input.is_action_pressed("up"):
		ymovement = ymovement + 0.05
	if Input.is_action_pressed("down"):
		ymovement = ymovement - 0.05
	if Input.is_action_pressed("right"):
		xmovement = xmovement + 0.05
	if Input.is_action_pressed("left"):
		xmovement = xmovement - 0.05
	var movementVector = Vector3(0,ymovement,xmovement)
	self.apply_impulse(Vector3(0,0,0),movementVector)

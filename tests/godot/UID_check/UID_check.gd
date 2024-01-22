extends Node

@export var resource_paths: PackedStringArray

func _ready():
	for path in resource_paths:
		print(path, ": ", ResourceLoader.get_resource_uid(path))

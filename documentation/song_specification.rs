iface song {
	String id;
	String banner_path;
	String background_path;
	track[] tracks;
	steps[] steps;
	tag[] tags;
}

iface track {
	String id;
	String name;
	String artist;
	String album;
	TrackSource source;
}

iface steps {
	String id;
	uint difficulty
	Difficulty preferred_difficulty;
	String stepper;
	StepSource source;
}

iface tag {
	String value;
}

enum Difficulty {
	Novice,
	Beginner,
	Standard,
	Heavy,
	Expert,
	Challenge,
	Oni
}

enum TrackSource {
	Local { path: String }
	// RemoteAudio
	// RemoteVideo
}

enum StepSource {
	Simfile { path: String, difficulty_name: String }
	// Bms
	// Bmson
	// SsmStepfile
}

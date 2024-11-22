.DEFAULT_GOAL := apk
.PHONY: jni apk run-on-device

gradle:
	cd app/android && gradle wrapper

jni:
	cargo ndk --target arm64-v8a -o ./app/android/app/src/main/jniLibs/ build --profile release --lib
	cargo ndk --target x86_64-linux-android -o ./app/android/app/src/main/jniLibs/ build --profile release --lib

apk: jni
	cd app/android && ./gradlew build --stacktrace
	zipalign -p 4 app/android/app/build/outputs/apk/release/app-release-unsigned.apk app/android/app/build/outputs/apk/release/app-release-unsigned-aligned.apk
	zipalign -c 4 app/android/app/build/outputs/apk/release/app-release-unsigned-aligned.apk
	apksigner sign --ks-key-alias app --ks my.keystore app/android/app/build/outputs/apk/release/app-release-unsigned-aligned.apk
	apksigner verify app/android/app/build/outputs/apk/release/app-release-unsigned-aligned.apk
	mv app/android/app/build/outputs/apk/release/app-release-unsigned-aligned.apk _builds/booklibrs_android.apk

run-on-device: jni
	adb uninstall local.booklibrs || true

	cd app/android && ./gradlew installDebug
	adb shell am start -n local.booklibrs/.MainActivity
	adb logcat -v color -s booklibrs *:e
	app/build/outputs/apk/release
clean:
	rm -rf app/android/app/src/main/jniLibs/
	rm -rf app/android/app/build/
	rm -rf app/android/.gradle/
	rm -rf target/
	rm -rf dist/android-28

build-all: apk
	cargo build --target x86_64-unknown-linux-gnu --release --bin app
	mv target/x86_64-pc-windows-gnu/release/app.exe _builds/booklibrs_windows.exe
	cargo build --target x86_64-pc-windows-gnu --release --bin app
	mv target/x86_64-unknown-linux-gnu/release/app _builds/booklibrs_linux
	cd app && trunk build --release
	rm -rf _builds/booklibrs_web
	mv app/dist _builds/booklibrs_web
	cp _builds/booklibrs_web docs -r
	cd docs && sed -i 's/\/app/\/booklibrs\/app/g' *.js
	cd docs && sed -i 's/\/app/\/booklibrs\/app/g' *.html
	echo All builds was successfull!
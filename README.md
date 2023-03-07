# pldi23-artifact

## Getting Started

### Prerequisites

This artifact provides a [docker](https://www.docker.com/) image with all the dependencies required to test the code. All the instructions assume you are using the image. To install docker please refer to their [documentation](https://docs.docker.com/).

### Overview of the Artifact

* [`/flux`](./flux): This directory contains a snapshot of the source code for Flux.
* [`/src`](./src): This directory contains the code for the examples in Section 2 of the paper and the code for the benchmarks used to compare Flux against Prusti in Section 5. It also contains some scripts to generate the results presented in Section 5.
* [`/prusti-dev`](./prusti-dev): This directory contains the snapshot of [Prusti](https://github.com/viperproject/prusti-dev) we used to compare against in Section 5.
* [`/liquid-fixpoint`](./liquid-fixpoint): This directory contains a snapshot of [Fixpoint](https://github.com/ucsd-progsys/liquid-fixpoint). Flux uses the fixpoint binary as an off-the-shelf horn constraint solver.
* [Dockerfile](Dockerfile): Instructions to build the docker image

### Docker Image

To build the image go to the directory containing the [`Dockerfile`](./Dockerfile) (the root of this artifact) and then run

```console
docker build --pull --rm -f "Dockerfile" -t fluxrs/pldi23-artifact:latest "."
```

This will install all the necessary dependencies inside the image and build Flux, Prusti and Fixpoint from source. It takes around 30min to complete in my laptop.

We also provide a [prebuilt image](https://hub.docker.com/repository/docker/fluxrs/pldi23-artifact) published in the docker registry. To download the image run

```console
docker pull fluxrs/pldi23-artifact
```

The final size of the image (uncompressed) is 7.42GB. It takes around 3 minutes to download in my laptop.

### Using the Image

To check everything is working, first run the following command to enter an interactive shell inside the docker image

```console
$ docker run --mount type=bind,source=$(pwd)/src,target=/src -it fluxrs/pldi23-artifact
root@72bc6d589d06:/src#
```

This invocation will mount the directory [src/](./src) inside the image and locate you in that directory. Note that you need to specify the absolute path to [`src/`](./src/) for it to work. You should be able to list the content of the directory.

```console
root@a02214363d68:/src# ls
benchmarks  count_lines.py  section2.rs  table1.py
```

To check the Flux binary is working properly run

```console
root@a02214363d68:/src# rustc-flux benchmarks/flux/kmp.rs --crate-type=rlib
```

This will run Flux on the [`kmp.rs`](./src/benchmarks/flux/kmp.rs) benchmark. Since the file doesn't contain any errors, the process should finish successfully producing an empty output.

Similarly, you should be able to run Prusti on the same benchmark.

```console
root@a02214363d68:/src# prusti-rustc benchmarks/prusti/kmp.rs --crate-type=rlib
  __          __        __  ___
 |__)  _\/_  |__) |  | /__`  |   ____\/_  |
 |      /\   |  \ \__/ .__/  |       /\   |

Prusti version: commit  <unknown>, built on 2023-02-28 06:16:47 UTC
Verification of 17 items...
Successful verification of 17 items
```

The command should finish without reporting any errors.

## Step-by-step Instructions

The rest of the README assumes all commands are run inside the docker image with the [src/](./src) directory mounted as described in the [Getting Started](#using-the-image) section. The instructions ask to modify files in this directory. Since the directory is mounted, you should be able to modify them from your host system and see the changes reflected inside the image. Alternatively, the image comes with `vim` and `nano` pre-installed in case you want to modify them from inside the image.

### Catching some Errors

The file [`section2.rs`](./src/section2.rs) contains the examples used in Section 2 of the paper. You can run Flux on this file using the following command:

```console
root@a02214363d68:/src# rustc-flux section2.rs
```

Since the file doesn't contain any errors the command finishes successfully without producing any output. Let's introduce some errors and see if Flux can catch them.

The function [`abs`](./src/section2.rs#L17) (Figure 1 in the paper) computes the absolute value of the `i32` input `x`. The output type specifies that the returned value is a non-negative `i32` whose value is at least as much as `x`. Suppose we forgot to negate `x` in the then branch [(L19)](./src/section2.rs#L19), i.e., let's modify the file as follows:

```diff
diff --git a/section2.rs b/section2.rs
index be5d809..307b55a 100644
--- a/section2.rs
+++ b/section2.rs
@@ -16,7 +16,7 @@ fn is_pos(n: i32) -> bool {
 #[flux::sig(fn(i32[@x]) -> i32{v: v >= x && v >= 0})]
 fn abs(x: i32) -> i32 {
     if x < 0 {
-        -x
+        x
     } else {
         x
     }
```

Running Flux on the modified file should produce the following output:

```console
root@c93e67d960cc:/src# rustc-flux section2.rs
error[FLUX]: postcondition might not hold
  --> section2.rs:19:9
   |
19 |         x
   |         ^

error: aborting due to previous error
```

As expected, Flux reports an error that says that the post-condition might not hold since we cannot ensure the result is greater than or equal to zero if we don't negate `x` in the then branch.

You can play around and introduce other errors and let Flux catch them. Here's a suggestion

```diff
diff --git a/src/section2.rs b/src/section2.rs
index be5d809..c1132ba 100644
--- a/src/section2.rs
+++ b/src/section2.rs
@@ -30,9 +30,7 @@ type Nat = i32;
 #[flux::sig(fn(&mut Nat))]
 fn decr(x: &mut Nat) {
     let y = *x;
-    if y > 0 {
-        *x = y - 1;
-    }
+    *x = y - 1;
 }
```

Running Flux on the modified file should produce the following output:

```console
root@9b32a8a21439:/src# rustc-flux section2.rs
error[FLUX]: assignment might be unsafe
  --> section2.rs:33:5
   |
33 |     *x = y - 1;
   |     ^^^^^^^^^^

error: aborting due to previous error
```

### Evaluation (Section 5)

The file [`table1.py`](./src/table1.py) contains a script to generate Table 1 in Section 5. By default, the script runs every benchmark 5 times and then reports the average verification time. It takes 20 min to run in my laptop. You can specify the number of repetitions with the `--repeat n` option. For example, to run each benchmark one time:

```console
./table1.py --repeat 1
```

The numbers will be slightly different from the table in the submission because some benchmarks have been modified. We will update it in the final version.

### More Examples

To explore more of Flux you can go the website <https://flux.programming.systems/> which contains a list with additional examples.

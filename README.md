# tfhe stand for : Torus-Fully Homomorphic Encryption ou Fully Homomorphic Encryption over the Torus
In this scheme one can encrypt messages like integers. So before encrypting a message we need to transform our message from "texts, images" etc to the corresponding integer [2].

The security of such a scheme is based on the Learning With Error which is called Quantum Resistant. The security depends on a distribution for keys and the small amount of noise we use to encrypt a message. It is said to be safe to use the Gaussian Distribution in instead of using others [1].

References : 

[1] Jean-Philippe Bossuat, Rosario Cammarota, Jung Hee Cheon, Ilaria Chillotti,
Benjamin R. Curtis, Wei Dai, Huijing Gong, Erin Hales, Duhyeong Kim,
Bryan Kumara, Changmin Lee, Xianhui Lu, Carsten Maple, Alberto
Pedrouzo-Ulloa, Rachel Player, Luis Antonio Ruiz Lopez, Yongsoo Song,
Donggeon Yhee, and Bahattin Yildiz. Security guidelines for implementing
homomorphic encryption. 2024.https://eprint.iacr.org/2024/463

[2] Ilaria Chillotti, Nicolas Gama, Mariya Georgieva, and Malika Izabachène.
Tfhe: Fast fully homomorphic encryption over the torus. 2018. https:
//eprint.iacr.org/2018/421.

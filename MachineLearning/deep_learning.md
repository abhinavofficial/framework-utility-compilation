# Introduction to Deep Learning

The fundamental unit or basic element in artificial neural networks is the “neuron,” based in part on a long history of analogies to the physiological neural brain elements and networks. This activity is designed for you to consolidate your knowledge about the different elements of a neuron.

$\alpha = f(z)$ where $z = \sum_{i=1}^nx_i*w_i + w_o$

- $\alpha$ is the output
- $f$ is the non-linear activation function
- $z$ is called the pre-activation function
- $x_i$ is input and $w_i$ is its corresponding weight
- $w_0$ is initial weight

Examples of non-linear function.

Sigmoid: $\sigma(z) = \frac{1}{(1 + e^{-z})}$ maps $z$ between $[0,1]$ and is useful for single class classification or probability models.

Hyperbolic tangent: $\tanh(z) = \frac{(e^z - e^{-z})}{(e^z + e^{-z})}$ maps $z$ between $[-1,1]$ and is useful for regression.

Rectified Linear Unit: $ReLU(z) = max(0,z)$ is useful for both regression and classification.

Loss function: $L(W,D)$ where $W$ signifies weight and $D$ signifies data, is optimized to get the best model output. For example, in regression sqared loss is often used while in classification negative log likelihood (NLL) is often used.

## Multi Layer Perceptrons (MLPs)

### How to train them?

Train them like a single neuron but with complicated and bookkeeping calculations. Supervised training is used with gradient descent to find small weight changes that improve the overall loss and cost function.

\begin{pmatrix}4\cr5\cr6\cr\end{pmatrix}

$x=A^0 \begin{pmatrix} W^1 \cr W^1_0 \cr\end{pmatrix}$

$A^L$ compared with training value, $Y$ to calculate the loss. We also calculate the gradient of the loss at each and every weight input. We then use backpropagation to distribute the loss at each layer.

### Backpropagation

- The calculation of the gradient proceeds backward throughout the network
- To distribute the overall loss to the change in each weight to help compensate for observed errors
- Requires care in the calculation of gradients for activation and loss functions
  - Matrix multiplications, calculations of gradients, backward propagation of data structures

Tensorflow can help set up this in an intituitive UI.

## Designs

### Autoencoders

Learn a simplified or encoded representation of a system to reproduce inputs to the outputs.

It is a system of encoder, $g(x)$ followed by a decoder, $h(a)$. Encoder reduces the input to a simpler, latent space or representation. Decoder seeks to reproduce the output. Unsupervised training is used to learn the encoder and decoder weights to achieve that match.

The goal for autoencoders is to create a latent representaion, in hope to create a small set of features A that explains the training data X. It is therefore, often used in conjunction with other ML methods. It can be used to perform clustering a dataset using latent space representation for example, in MNIST Image Dataset.

Any sophisticated neural network is using these implicitly trying to learn good features and combine those features to achieve some purpose such as classification and prediction.

Variational Autoencoders (VAEs) combine the neural network idea with a probabilistic latent reprsentation, $a \sim N(\mu(X), \sum(X))$. Training is done with additional loss terms to evolve the latest representation while staying near to some prior distribution. This latent representation can be used as random sample to generate various output - this can be used to generate fake images or text.

### Convolutional Neural Networks (CNNs)

Neural network design for image processing. 2D image with spatial location.

A standard signal processing method is a convolution, which can be thought of as a weighted moving average of some input, with the weights being given by a filter.

Example - 9x9 input can be convoluted to a 3x3 filter stride, which can be fed into a ReLU with some offset to create a 3x3 output, if the goal was to find a ring with gray scale at the outside with white inside.

The algorithm is meant to find weights that help with extracting relevant features.

**Max pooling layer** ??
**Tensor layer** ??

Other layers in CNNs:

- Flatten layer: Convert the 2D or tensor result into a single 1D vector before feeding it to a fully connected MLP layer
- Norm: Normalize the values within the layers, to keep propagating values in a layer with some range.

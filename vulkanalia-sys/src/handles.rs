// SPDX-License-Identifier: Apache-2.0

// DO NOT EDIT.
//
// This file has been generated by the Kotlin project in the `generator`
// directory from a Vulkan API registry.

use std::fmt;
use std::hash::Hash;

use crate::enums::ObjectType;

/// A Vulkan handle type.
pub trait Handle: Copy + Clone + fmt::Debug + PartialEq + Eq + Hash + Default + Sized {
    /// The underlying type for this handle type.
    type Repr;

    /// The object type for this handle type.
    const TYPE: ObjectType;

    /// Constructs a null instance of this handle type.
    fn null() -> Self;

    /// Constructs an instance of this handle type with the supplied underlying value.
    fn from_raw(value: Self::Repr) -> Self;

    /// Gets the underlying value for this handle.
    fn as_raw(self) -> Self::Repr;

    /// Returns whether this handle is a null handle.
    fn is_null(self) -> bool;
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureKHR.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct AccelerationStructureKHR(u64);

impl Handle for AccelerationStructureKHR {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::ACCELERATION_STRUCTURE_KHR;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for AccelerationStructureKHR {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for AccelerationStructureKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AccelerationStructureKHR({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuffer.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Buffer(u64);

impl Handle for Buffer {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::BUFFER;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Buffer {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferView.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BufferView(u64);

impl Handle for BufferView {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::BUFFER_VIEW;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for BufferView {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for BufferView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BufferView({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBuffer.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CommandBuffer(usize);

impl Handle for CommandBuffer {
    type Repr = usize;

    const TYPE: ObjectType = ObjectType::COMMAND_BUFFER;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for CommandBuffer {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for CommandBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CommandBuffer({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPool.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CommandPool(u64);

impl Handle for CommandPool {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::COMMAND_POOL;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for CommandPool {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for CommandPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CommandPool({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportCallbackEXT.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DebugReportCallbackEXT(u64);

impl Handle for DebugReportCallbackEXT {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::DEBUG_REPORT_CALLBACK_EXT;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for DebugReportCallbackEXT {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for DebugReportCallbackEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DebugReportCallbackEXT({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerEXT.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DebugUtilsMessengerEXT(u64);

impl Handle for DebugUtilsMessengerEXT {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::DEBUG_UTILS_MESSENGER_EXT;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for DebugUtilsMessengerEXT {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for DebugUtilsMessengerEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DebugUtilsMessengerEXT({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeferredOperationKHR.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DeferredOperationKHR(u64);

impl Handle for DeferredOperationKHR {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::DEFERRED_OPERATION_KHR;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for DeferredOperationKHR {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for DeferredOperationKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeferredOperationKHR({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPool.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorPool(u64);

impl Handle for DescriptorPool {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::DESCRIPTOR_POOL;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for DescriptorPool {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for DescriptorPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DescriptorPool({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSet.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorSet(u64);

impl Handle for DescriptorSet {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::DESCRIPTOR_SET;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for DescriptorSet {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for DescriptorSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DescriptorSet({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayout.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorSetLayout(u64);

impl Handle for DescriptorSetLayout {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::DESCRIPTOR_SET_LAYOUT;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for DescriptorSetLayout {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for DescriptorSetLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DescriptorSetLayout({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplate.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorUpdateTemplate(u64);

impl Handle for DescriptorUpdateTemplate {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::DESCRIPTOR_UPDATE_TEMPLATE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for DescriptorUpdateTemplate {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for DescriptorUpdateTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DescriptorUpdateTemplate({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDevice.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Device(usize);

impl Handle for Device {
    type Repr = usize;

    const TYPE: ObjectType = ObjectType::DEVICE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Device {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Device({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemory.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DeviceMemory(u64);

impl Handle for DeviceMemory {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::DEVICE_MEMORY;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for DeviceMemory {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for DeviceMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceMemory({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayKHR.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DisplayKHR(u64);

impl Handle for DisplayKHR {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::DISPLAY_KHR;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for DisplayKHR {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for DisplayKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DisplayKHR({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDisplayModeKHR.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DisplayModeKHR(u64);

impl Handle for DisplayModeKHR {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::DISPLAY_MODE_KHR;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for DisplayModeKHR {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for DisplayModeKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DisplayModeKHR({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEvent.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Event(u64);

impl Handle for Event {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::EVENT;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Event {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFence.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Fence(u64);

impl Handle for Fence {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::FENCE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Fence {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Fence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fence({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebuffer.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Framebuffer(u64);

impl Handle for Framebuffer {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::FRAMEBUFFER;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Framebuffer {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Framebuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Framebuffer({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImage.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Image(u64);

impl Handle for Image {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::IMAGE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Image {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageView.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageView(u64);

impl Handle for ImageView {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::IMAGE_VIEW;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for ImageView {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for ImageView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImageView({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndirectCommandsLayoutNV.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct IndirectCommandsLayoutNV(u64);

impl Handle for IndirectCommandsLayoutNV {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::INDIRECT_COMMANDS_LAYOUT_NV;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for IndirectCommandsLayoutNV {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for IndirectCommandsLayoutNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IndirectCommandsLayoutNV({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInstance.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Instance(usize);

impl Handle for Instance {
    type Repr = usize;

    const TYPE: ObjectType = ObjectType::INSTANCE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Instance {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instance({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPerformanceConfigurationINTEL.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PerformanceConfigurationINTEL(u64);

impl Handle for PerformanceConfigurationINTEL {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::PERFORMANCE_CONFIGURATION_INTEL;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for PerformanceConfigurationINTEL {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for PerformanceConfigurationINTEL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PerformanceConfigurationINTEL({:p})",
            self.0 as *const u8
        )
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PhysicalDevice(usize);

impl Handle for PhysicalDevice {
    type Repr = usize;

    const TYPE: ObjectType = ObjectType::PHYSICAL_DEVICE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for PhysicalDevice {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for PhysicalDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PhysicalDevice({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipeline.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pipeline(u64);

impl Handle for Pipeline {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::PIPELINE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Pipeline {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pipeline({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCache.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PipelineCache(u64);

impl Handle for PipelineCache {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::PIPELINE_CACHE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for PipelineCache {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for PipelineCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PipelineCache({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayout.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PipelineLayout(u64);

impl Handle for PipelineLayout {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::PIPELINE_LAYOUT;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for PipelineLayout {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for PipelineLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PipelineLayout({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotEXT.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PrivateDataSlotEXT(u64);

impl Handle for PrivateDataSlotEXT {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::PRIVATE_DATA_SLOT_EXT;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for PrivateDataSlotEXT {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for PrivateDataSlotEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PrivateDataSlotEXT({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPool.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct QueryPool(u64);

impl Handle for QueryPool {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::QUERY_POOL;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for QueryPool {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for QueryPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QueryPool({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueue.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Queue(usize);

impl Handle for Queue {
    type Repr = usize;

    const TYPE: ObjectType = ObjectType::QUEUE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Queue {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Queue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Queue({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPass.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct RenderPass(u64);

impl Handle for RenderPass {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::RENDER_PASS;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for RenderPass {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for RenderPass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RenderPass({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampler.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Sampler(u64);

impl Handle for Sampler {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::SAMPLER;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Sampler {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Sampler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sampler({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversion.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SamplerYcbcrConversion(u64);

impl Handle for SamplerYcbcrConversion {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::SAMPLER_YCBCR_CONVERSION;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for SamplerYcbcrConversion {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for SamplerYcbcrConversion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SamplerYcbcrConversion({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphore.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Semaphore(u64);

impl Handle for Semaphore {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::SEMAPHORE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for Semaphore {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Semaphore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Semaphore({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModule.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ShaderModule(u64);

impl Handle for ShaderModule {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::SHADER_MODULE;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for ShaderModule {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for ShaderModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShaderModule({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceKHR.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SurfaceKHR(u64);

impl Handle for SurfaceKHR {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::SURFACE_KHR;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for SurfaceKHR {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for SurfaceKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SurfaceKHR({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSwapchainKHR.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SwapchainKHR(u64);

impl Handle for SwapchainKHR {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::SWAPCHAIN_KHR;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for SwapchainKHR {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for SwapchainKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SwapchainKHR({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheEXT.html>
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ValidationCacheEXT(u64);

impl Handle for ValidationCacheEXT {
    type Repr = u64;

    const TYPE: ObjectType = ObjectType::VALIDATION_CACHE_EXT;

    #[inline]
    fn null() -> Self {
        Self(0)
    }

    #[inline]
    fn from_raw(value: Self::Repr) -> Self {
        Self(value)
    }

    #[inline]
    fn as_raw(self) -> Self::Repr {
        self.0
    }

    #[inline]
    fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Default for ValidationCacheEXT {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for ValidationCacheEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValidationCacheEXT({:p})", self.0 as *const u8)
    }
}

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureNV.html>
pub type AccelerationStructureNV = AccelerationStructureKHR;
/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorUpdateTemplateKHR.html>
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;
/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerYcbcrConversionKHR.html>
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;

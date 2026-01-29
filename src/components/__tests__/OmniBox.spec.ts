import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import OmniBox from '../OmniBox.vue'

describe('OmniBox', () => {
  it('renders properly', () => {
    const wrapper = mount(OmniBox)
    expect(wrapper.find('input').exists()).toBe(true)
  })

  it('updates input value', async () => {
    const wrapper = mount(OmniBox)
    const input = wrapper.find('input')
    await input.setValue('test command')
    expect(input.element.value).toBe('test command')
  })

  it('clears input on submit', async () => {
    const wrapper = mount(OmniBox)
    const input = wrapper.find('input')
    await input.setValue('test command')
    await input.trigger('keydown', { key: 'Enter' })
    expect(input.element.value).toBe('')
  })
})

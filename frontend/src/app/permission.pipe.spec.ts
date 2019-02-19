import { PermissionPipe } from './permission.pipe';

describe('PermissionPipe', () => {
  it('create an instance', () => {
    const pipe = new PermissionPipe();
    expect(pipe).toBeTruthy();
  });
});

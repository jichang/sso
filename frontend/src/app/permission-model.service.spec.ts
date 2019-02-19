import { TestBed } from '@angular/core/testing';

import { PermissionModelService } from './permission-model.service';

describe('PermissionModelService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: PermissionModelService = TestBed.get(PermissionModelService);
    expect(service).toBeTruthy();
  });
});
